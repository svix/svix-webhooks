set dotenv-load
set default-list

import "../.justfile-common-rust"

PYTHON := "python3.14"
HERE := justfile_directory()
ENVFILE := HERE / "svix-server" / "development.env"

# Run the svix server
run *args='':
    cargo run -p svix-server -- {{ args }}

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
[no-exit-message]
dc *args='':
    docker compose -p svix-oss -f {{ HERE / "testing-docker-compose.yml" }} {{ args }}
