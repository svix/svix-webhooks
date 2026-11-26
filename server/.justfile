set dotenv-load
set default-list

import "../.justfile-common-rust"

PYTHON := "python3.14"
HERE := justfile_directory()
ENVFILE := HERE / "svix-server" / "development.env"

# Run the svix server
run *args='':
    cargo run -p svix-server -- {{ args }}

# Run the svix server, re-executing on code changes
watch-run:
    watchexec -w svix-server/ -w svix-server-derive/ -w Cargo.toml -w Cargo.lock --debounce 500ms cargo run -p svix-server

# Generates a new `openapi.json`. Pass `--check` to compare the spec that would be generated to the current version of the file. Make sure to update `codegen/lib-openapi.json` first.
[no-exit-message]
generate-openapi:
    bash generate-openapi.sh

# Regenerate all client libraries
[no-exit-message]
[working-directory(HERE / "..")]
codegen:
    {{ PYTHON }} regen_openapi.py

# Invokes `docker compose` with the testing configuration in the server directory
[group("docker")]
[no-exit-message]
dc *args='':
    docker compose -p svix-oss -f {{ HERE / "testing-docker-compose.yml" }} {{ args }}

# Create a new migration
[group("db")]
[no-exit-message]
add-migration description:
    sqlx migrate add -r {{ description }}

# Runs a series of checks on both formatting, syntax, idioms, etc. Auto-fixes whatever it can, so best to commit first
[group("lint")]
[no-quiet]
lint: check clippy fmt-check

# Runs a series of checks on both formatting, syntax, idioms, etc. Auto-fixes whatever it can, so best to commit first
[group("lint")]
[no-quiet]
lint-fix: check (clippy "--fix --allow-dirty") fmt
