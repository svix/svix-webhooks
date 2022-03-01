#!/usr/bin/env bash

set -ex

cd svix

if [ ! -d "./internal" ]; then
    openapi-python-client generate --path ../../openapi.json --custom-template-path=../templates --config ../openapi-generator-config.json
else
    openapi-python-client update --path ../../openapi.json --custom-template-path=../templates --config ../openapi-generator-config.json
fi

cd internal

touch __init__.py