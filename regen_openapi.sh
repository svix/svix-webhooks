#!/bin/bash

set -exo pipefail

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > openapi.json
fi

# For some languages, write a separate OpenAPI spec file where optional fields
# of non-`Patch` schemas are set to not be nullable, so the codegen doesn't wrap
# the struct fields in double options.
# Rust's serde and Go's encoding/json will respect both in deserialization
# anyways, but this frees users from having to think about the difference
# between absence of the field or setting it to `null` (which is only
# significant for patch request bodies).
jq --indent 4 '.components.schemas |= with_entries(
        if .key | endswith("Patch") then .
        else
            (.value.required // []) as $required |
            if .value | has("properties") then
                .value.properties |= with_entries(
                    if .key | IN($required[]) then .
                    else del(.value.nullable)
                    end
                )
            else .
            end
        end
    )' \
    < openapi.json \
    | tee go/openapi.json \
    > rust/openapi.json

yarn openapi-generator-cli generate -i openapi.json -g typescript -o javascript/src/openapi -c javascript/openapi-generator-config.json --type-mappings=set=Array -t javascript/templates

# Cleanup previous codegen, allowing us to spot removals.
# If the removals are expected, stage them eg. `git add -u`, then commit them.
rm -f go/internal/openapi/*.go
yarn openapi-generator-cli generate -i go/openapi.json -g go -o go/internal/openapi -c go/openapi-generator-config.json -t go/templates

yarn openapi-generator-cli generate -i openapi.json -g java -o java/lib/generated/openapi -c java/openapi-generator-config.json -t java/templates

yarn openapi-generator-cli generate -i openapi.json -g kotlin -o kotlin/lib/generated/openapi -c kotlin/openapi-generator-config.json -t kotlin/templates

yarn openapi-generator-cli generate -i openapi.json -g ruby -o ruby -c ruby/openapi-generator-config.json -t ruby/templates

yarn openapi-generator-cli generate -i openapi.json -g csharp -o csharp/ -c csharp/openapi-generator-config.json --global-property apis,models,supportingFiles,apiTests=false,apiDocs=false,modelTests=false,modelDocs=false

# Cleanup previous codegen, allowing us to spot removals.
# If the removals are expected, stage them eg. `git add -u`, then commit them.
rm -rf rust/src/{apis,models}
yarn openapi-generator-cli generate -i rust/openapi.json -g rust -o rust/ -c rust/openapi-generator-config.json -t rust/templates

echo Note: Python generation is not executed automatically.
