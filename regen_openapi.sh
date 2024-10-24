#!/bin/sh

set -ex

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > openapi.json
fi

yarn openapi-generator-cli generate -i openapi.json -g typescript -o javascript/src/openapi -c javascript/openapi-generator-config.json --type-mappings=set=Array -t javascript/templates

# Cleanup previous codegen, allowing us to spot removals.
# If the removals are expected, stage them eg. `git add -u`, then commit them.
rm -f go/internal/openapi/*.go
yarn openapi-generator-cli generate -i openapi.json -g go -o go/internal/openapi -c go/openapi-generator-config.json -t go/templates


yarn openapi-generator-cli generate -i openapi.json -g java -o java/lib/generated/openapi -c java/openapi-generator-config.json -t java/templates

yarn openapi-generator-cli generate -i openapi.json -g kotlin -o kotlin/lib/generated/openapi -c kotlin/openapi-generator-config.json -t kotlin/templates

yarn openapi-generator-cli generate -i openapi.json -g ruby -o ruby -c ruby/openapi-generator-config.json -t ruby/templates

yarn openapi-generator-cli generate -i openapi.json -g csharp -o csharp/ -c csharp/openapi-generator-config.json --global-property apis,models,supportingFiles,apiTests=false,apiDocs=false,modelTests=false,modelDocs=false

yarn openapi-generator-cli generate -i openapi.json -g rust -o rust/ -c rust/openapi-generator-config.json -t rust/templates

echo Note: Python generation is not executed automatically.
