#!/bin/bash

#
# variables
#

# AWS variables
AWS_PROFILE=default
AWS_REGION=ap-northeast-1
# project name
PROJECT_NAME=rust-lambda-example


# the directory containing the script file
dir="$(cd "$(dirname "$0")"; pwd)"
cd "$dir"

# log color
WHITE='\e[30;47m'; RED='\e[48;5;196m'; BLUE='\e[48;5;28m'; YELLO='\e[48;5;220m'; 
NC='\033[0m'; # No Color
log()     {  printf "${WHITE} ${1} ${NC} ${@:2}\n";}
error()   {  printf "${RED} ${1} ${NC} ${@:2}\n";}
info()    {  printf "${BLUE} ${1} ${NC} ${@:2}\n";}
warn()    {  printf "${YELLO} ${1} ${NC} ${@:2}\n";}

# log $1 in underline then $@ then a newline
under() {
    local arg=$1
    shift
    echo -e "\033[0;4m${arg}\033[0m ${@}"
    echo
}

usage() {
    under usage 'call the Makefile directly: make dev
      or invoke this file directly: ./make.sh dev'
}

run() {
    cd ../
    cargo lambda watch
}

test-get() {
    cargo lambda invoke --data-file ../src/request_get.json
}

test-post() {
    cargo lambda invoke --data-file ../src/request_post.json
}

build() {
    cargo build
}

get() {
 curl $YOUR_LAMBDA_FUNCTION_URL/name\?first_name\="Mary"\&last_name\="Smith"
}

post() {
    curl --location --request POST "${YOUR_LAMBDA_FUNCTION_URL}/name" \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "first_name": "John",
        "last_name": "Smith"
    }'
}

# if `$1` is a function, execute it. Otherwise, print usage
# compgen -A 'function' list all declared functions
# https://stackoverflow.com/a/2627461
FUNC=$(compgen -A 'function' | grep $1)
[[ -n $FUNC ]] && { eval $1; } || usage;
exit 0
