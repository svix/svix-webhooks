#!/bin/bash
# Generates OpenAPI spec

set -eo pipefail

export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
export SVIX_JWT_SECRET="test value"
export SVIX_LOG_LEVEL="info"
export SVIX_WHITELIST_SUBNETS="[127.0.0.1/32]"
export SVIX_QUEUE_TYPE="memory"
export SVIX_CACHE_TYPE="none"
export SVIX_REDIS_DSN="redis://localhost:6379"

server_path=$(dirname "$0")
openapi_path=$server_path/openapi.json
# Do not output the entire thing
output=$(cargo run --manifest-path "$server_path"/Cargo.toml -- generate-openapi | jq -S --indent 4 '.openapi = "3.0.2" | .info.version = "1.1.1" | .info.description = ""')

if [[ "$1" = "--check" ]]; then
    # Do not quit immediately if the diff fails, we check status afterwards
    set +e
    if ! diff_result=$(echo "$output" | diff -C5 "$openapi_path" -); then
        echo "===== Uncommitted difference in OpenAPI spec generation output! ====="
        echo
        echo "$diff_result"
        echo
        echo "===== If these changes were intended then run locally $0 and add the changes to the commit. ====="
        exit 1
    fi
    echo "No differences found"
else
    echo "$output" > "$openapi_path"
    echo "OpenAPI specification written to $openapi_path"
fi
