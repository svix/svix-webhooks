#!/bin/sh

set -ex

curl $1 | python -m json.tool > openapi.json

yarn openapi-generator-cli generate -i openapi.json -g python -o python/ -c python/openapi-generator-config.json

yarn openapi-generator-cli generate -i openapi.json -g typescript -o javascript/openapi -c javascript/openapi-generator-config.json
