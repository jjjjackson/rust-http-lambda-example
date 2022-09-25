use lambda_http::{
    http::Method, http::StatusCode, service_fn, tower::ServiceBuilder, Body, Error, Request,
    RequestExt, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Define a layer to inject CORS headers
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any);

    let handler = ServiceBuilder::new()
        // Add the CORS layer to the service
        .layer(cors_layer)
        .service(service_fn(func));

    lambda_http::run(handler).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Name {
    first_name: String,
    last_name: String,
}

async fn func(event: Request) -> Result<Response<Body>, Error> {
    info!(
        tag = "func",
        path = format!("{}", event.uri().path()),
        method = format!("{}", event.method()),
    );
    let (status, response) = match event.uri().path() {
        "/name" => match *event.method() {
            Method::GET => get_name_handler(&event).await,
            Method::POST => post_name_handler(&event).await,
            _ => not_found(),
        },
        _ => not_found(),
    }
    .unwrap_or((StatusCode::INTERNAL_SERVER_ERROR, Value::Null));

    Ok(Response::builder()
        .status(status)
        .body(Body::Text(response.to_string()))?)
}

fn not_found() -> Result<(StatusCode, Value), Error> {
    Ok((StatusCode::NOT_FOUND, Value::Null))
}

async fn get_name_handler(event: &Request) -> Result<(StatusCode, Value), Error> {
    let tag = "get name";
    let query = event.query_string_parameters();
    if let (Some(first_name), Some(last_name)) =
        (query.first("first_name"), query.first("last_name"))
    {
        let params = Name {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };
        let resp =
            json!({ "message": format!("Hello {} {}", params.first_name, params.last_name) });
        Ok((StatusCode::OK, resp))
    } else {
        error!(
            tag,
            query = format!("{query:?}"),
            message = "params deserializing failed"
        );
        Ok((StatusCode::FORBIDDEN, Value::Null))
    }
}

async fn post_name_handler(event: &Request) -> Result<(StatusCode, Value), Error> {
    let tag = "update name";
    let params = match event.body() {
        Body::Text(text) => {
            info!(tag, text);
            serde_json::from_str(text).unwrap_or_else(|error| {
                error!(
                    tag,
                    text,
                    message = "parse failed",
                    error = format!("{error:?}")
                );
                Value::Null
            })
        }
        _ => Value::Null,
    };

    if params == Value::Null {
        return Ok((StatusCode::FORBIDDEN, params));
    }

    let resp =
        json!({ "message": format!("Hello {}", params["first_name"].as_str().unwrap_or("world")) });

    Ok((StatusCode::OK, resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::include_str;

    #[tokio::test]
    async fn test_get() {
        let input = include_str!("request_get.json");

        let request = lambda_http::request::from_str(&input.to_string())
            .expect("failed to create request")
            .with_raw_http_path("localhost/name");

        let response = func(request).await.expect("failed to handle request");
        assert_eq!(response.status(), 200);
        if let Body::Text(text) = response.body() {
            let value = serde_json::from_str(&text).unwrap_or(Value::Null);
            assert_eq!(value, json!({"message": "Hello Mary Smith"}));
        }
    }

    #[tokio::test]
    async fn test_post() {
        let input = include_str!("request_post.json");

        let request = lambda_http::request::from_str(&input.to_string())
            .expect("failed to create request")
            .with_raw_http_path("localhost/name");

        let response = func(request).await.expect("failed to handle request");
        assert_eq!(response.status(), 200);
        if let Body::Text(text) = response.body() {
            let value = serde_json::from_str(&text).unwrap_or(Value::Null);
            assert_eq!(value, json!({"message": "Hello Mary"}));
        }
    }
}
