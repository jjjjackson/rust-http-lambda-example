.SILENT:

help:
	{ grep --extended-regexp '^[a-zA-Z_-]+:.*#[[:space:]].*$$' $(MAKEFILE_LIST) || true; } \
	| awk 'BEGIN { FS = ":.*#[[:space:]]*" } { printf "\033[1;32m%-25s\033[0m%s\n", $$1, $$2 }'

run: # start a local lambda server
	./scripts/make.sh run

test-get: # try get endpoint
	./scripts/make.sh test-get

test-post: # try post endpoint
	./scripts/make.sh test-post

build: # build for release
	./scripts/make.sh build

deploy: # deploy with cargo-lambda
	./scripts/deploy.sh deploy-with-cargo-lambda

get: # try get endpoint
	./scripts/make.sh get

post: # try post endpoint
	./scripts/make.sh post
