#!/bin/bash

set -eo pipefail

OPENAPI_GIT_REV='272125558d6ac4718bdc87b1652e5d4122b69f19'

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > lib-openapi.json
fi

if ! command -v openapi-codegen >/dev/null; then
    if [[ -z "$GITHUB_WORKFLOW" ]]; then
        echo openapi-codegen is not installed. install using
        echo "cargo install --git https://github.com/svix/openapi-codegen --rev $OPENAPI_GIT_REV --locked"
        exit 1
    else
        cargo install --git https://github.com/svix/openapi-codegen --rev $OPENAPI_GIT_REV --locked
    fi
fi

# Rust
(
    # Print commands we run
    set -x

    openapi-codegen generate \
        --template rust/templates/api_resource.rs.jinja \
        --input-file lib-openapi.json \
        --output-dir rust/src/api
    openapi-codegen generate \
        --template rust/templates/component_type.rs.jinja \
        --input-file lib-openapi.json \
        --output-dir rust/src/models

    # Remove APIs we may not (yet) want to expose
    rm rust/src/api/{environment,health}.rs
)

# skip everything else for now
exit

# JavaScript
(
    # Print commands we run
    set -x

    openapi-codegen generate \
        --template javascript/templates/api_resource.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/api
    openapi-codegen generate \
        --template javascript/templates/component_type_summary.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/models
    openapi-codegen generate \
        --template javascript/templates/component_type.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/models
)


# CLI
(
    # Print commands we run
    set -x

    openapi-codegen generate \
        --template svix-cli/templates/api_resource.rs.jinja \
        --input-file lib-openapi.json \
        --output-dir svix-cli/src/cmds/api

    # Our CLI templates currently output some unused imports. Get rid of them.
    cargo fix --manifest-path svix-cli/Cargo.toml --allow-dirty
    # `cargo fix` can leave the source in an inconsistently-formatted state.
    cargo fmt --manifest-path svix-cli/Cargo.toml

    # Remove APIs we may not (yet) want to expose
    rm svix-cli/src/cmds/api/{background_task,environment,health,operational_webhook_endpoint,statistics}.rs
)

# Python
(
    # Print commands we run
    set -x

    #openapi-codegen generate \
    #    --template python/templates/api_summary.py.jinja \
    #    --input-file lib-openapi.json \
    #    --output-dir python/svix/api
    openapi-codegen generate \
        --template python/templates/api_resource.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/api
    openapi-codegen generate \
        --template python/templates/component_type_summary.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/models
    openapi-codegen generate \
        --template python/templates/component_type.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/models

    # Remove APIs we may not (yet) want to expose
    rm python/svix/api/{environment,health}.py
)

