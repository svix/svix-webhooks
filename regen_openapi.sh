#!/bin/sh

set -ex

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > openapi.json
fi

yarn openapi-generator-cli generate -i openapi.json -g python -o python/ -c python/openapi-generator-config.json -t python/templates

yarn openapi-generator-cli generate -i openapi.json -g typescript -o javascript/src/openapi -c javascript/openapi-generator-config.json

yarn openapi-generator-cli generate -i openapi.json -g go -o go/internal/openapi -c go/openapi-generator-config.json -t go/templates

yarn openapi-generator-cli generate -i openapi.json -g java -o java/lib/generated/openapi -c java/openapi-generator-config.json -t java/templates

yarn openapi-generator-cli generate -i openapi.json -g kotlin -o kotlin/lib/generated/openapi -c kotlin/openapi-generator-config.json -t kotlin/templates
