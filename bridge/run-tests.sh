#!/bin/bash

if [[ -z "$TEST_COMMAND" ]]; then
    if [[ -z "$CARGO_HOME" ]]; then
        CARGO_HOME="$HOME/.cargo"
    fi

    if command -v cargo-nextest || [[ -e "$CARGO_HOME/bin/cargo-nextest" ]]; then
        TEST_COMMAND="cargo nextest run"
    else
        TEST_COMMAND="cargo test"
    fi
fi

AWS_DEFAULT_REGION="elasticmq" \
AWS_ACCESS_KEY_ID="x" \
AWS_SECRET_ACCESS_KEY="x" \
PUBSUB_EMULATOR_HOST=localhost:8085 \
PUBSUB_PROJECT_ID=local-project \
${TEST_COMMAND} --all-features -- "$@"
