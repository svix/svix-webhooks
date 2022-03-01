#!/usr/bin/env bash

set -ex

cd svix

if [ ! -d "./internal" ]; then
    COMMAND="generate"
else
    COMMAND="update"
fi

openapi-python-client $COMMAND --path ../../openapi.json --custom-template-path=../templates --config ../openapi-generator-config.json

cd internal

touch __init__.py