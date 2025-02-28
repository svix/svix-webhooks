#!/bin/bash

set -eo pipefail

OPENAPI_GIT_REV='272125558d6ac4718bdc87b1652e5d4122b69f19'
OPENAPI_CODEGEN_IMAGE='ghcr.io/svix/openapi-codegen:latest'
REPO_ROOT=$(git rev-parse --show-toplevel)

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool >lib-openapi.json
fi

if ! command -v podman >/dev/null; then
    if ! command -v docker >/dev/null; then
        echo Please install docker or podman to run the codegen
        exit 1
    else
        DOCKER_BIN="docker"
    fi
else
    DOCKER_BIN="podman"
fi

run_generate() {
    # Pass in the `docker container run ...` command
    # MAKE SURE  `--output-dir ...` IS THE LAST ARGUMENT
    #       This function assumes the last argument is the output dir
    #       The output dir is used to copy the generated file from the container
    # $1 is the docker binary
    # steps
    # 1. run the container
    # 2. wait for it to exit
    # 3. copy the generated files
    # 4. remove the container
    CONTAINER_ID=$(${@:1})
    EXIT_STATUS=$($1 container wait $CONTAINER_ID)
    $1 container logs $CONTAINER_ID
    if [[ $EXIT_STATUS != 0 ]]; then
        exit $EXIT_STATUS
    fi
    $1 cp $CONTAINER_ID:/app/${@: -1}/. ${@: -1} >/dev/null
    $1 container rm $CONTAINER_ID >/dev/null
}

if [[ $DEBUG != "" ]]; then
    set -x
fi

# JavaScript
(
    echo Generating JavaScript lib

    GENERATE="$DOCKER_BIN container run -dit --workdir /app \
        --mount type=bind,src=$REPO_ROOT/lib-openapi.json,dst=/app/lib-openapi.json,ro \
        --mount type=bind,src=$REPO_ROOT/javascript/templates,dst=/app/javascript/templates,ro \
        $OPENAPI_CODEGEN_IMAGE \
        openapi-codegen generate --create-file-parents"

    run_generate $GENERATE \
        --template javascript/templates/api_resource.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/api

    run_generate $GENERATE \
        --template javascript/templates/component_type_summary.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/models

    run_generate $GENERATE \
        --template javascript/templates/component_type.ts.jinja \
        --input-file lib-openapi.json \
        --output-dir javascript/src/models
)

# Rust
(
    echo Generating Rust lib

    GENERATE="$DOCKER_BIN container run -dit --workdir /app \
        --mount type=bind,src=$REPO_ROOT/lib-openapi.json,dst=/app/lib-openapi.json,ro \
        --mount type=bind,src=$REPO_ROOT/rust/templates,dst=/app/rust/templates,ro \
        --mount type=bind,src=$REPO_ROOT/rust/.rustfmt.toml,dst=/app/.rustfmt.toml,ro \
        $OPENAPI_CODEGEN_IMAGE \
        openapi-codegen generate --create-file-parents"

    run_generate $GENERATE \
        --template rust/templates/api_resource.rs.jinja \
        --input-file lib-openapi.json \
        --output-dir rust/src/api

    run_generate $GENERATE \
        --template rust/templates/component_type.rs.jinja \
        --input-file lib-openapi.json \
        --output-dir rust/src/models

    # Remove APIs we may not (yet) want to expose
    rm rust/src/api/{environment,health}.rs
)

# CLI

(
    echo Generating svix-cli

    GENERATE="$DOCKER_BIN container run -dit --workdir /app \
        --mount type=bind,src=$REPO_ROOT/lib-openapi.json,dst=/app/lib-openapi.json,ro \
        --mount type=bind,src=$REPO_ROOT/svix-cli/templates,dst=/app/svix-cli/templates,ro \
        --mount type=bind,src=$REPO_ROOT/svix-cli/.rustfmt.toml,dst=/app/.rustfmt.toml,ro \
        $OPENAPI_CODEGEN_IMAGE \
        openapi-codegen generate --create-file-parents"

    run_generate $GENERATE \
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
    echo Generating Python lib

    GENERATE="$DOCKER_BIN container run -dit --workdir /app \
        --mount type=bind,src=$REPO_ROOT/lib-openapi.json,dst=/app/lib-openapi.json,ro \
        --mount type=bind,src=$REPO_ROOT/python/templates,dst=/app/python/templates,ro \
        $OPENAPI_CODEGEN_IMAGE \
        openapi-codegen generate --create-file-parents"

    #openapi-codegen generate \
    #    --template python/templates/api_summary.py.jinja \
    #    --input-file lib-openapi.json \
    #    --output-dir python/svix/api

    run_generate $GENERATE \
        --template python/templates/api_resource.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/api
    run_generate $GENERATE \
        --template python/templates/component_type_summary.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/models
    run_generate $GENERATE \
        --template python/templates/component_type.py.jinja \
        --input-file lib-openapi.json \
        --output-dir python/svix/models

    # Remove APIs we may not (yet) want to expose
    rm python/svix/api/{environment,health}.py
)
