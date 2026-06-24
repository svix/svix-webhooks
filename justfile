set dotenv-load

PYTHON := "python3.14"
HERE := justfile_directory()
SERVER := HERE / "server"
ENVFILE := SERVER / "svix-server" / "development.env"

# Run `cargo check` for the 'server' codebase.
[group('server')]
[working-directory(SERVER)]
check-server:
    cargo check --all-targets

# Run `cargo clippy` for the 'server' codebase.
[group('server')]
[working-directory(SERVER)]
clippy-server *args='':
    cargo clippy --all-targets {{ args }} -- -D warnings

# Run `cargo clippy` for the 'server' codebase.
[group('server')]
[working-directory(SERVER)]
run-server *args='':
    cargo run -p svix-server -- {args}

# Reformat both this justfile and the 'server' sources.
[no-exit-message]
fmt: fmt-server
    just --fmt --unstable

# Reformat rust server sources
[group('server')]
[working-directory(SERVER)]
fmt-server:
    cargo +nightly fmt --all
    cargo sort --workspace
    cargo machete --fix

# Generates a new `openapi.json`. Pass `--check` to compare the spec that would be generated to the current version of the file.
#
# Make sure to update `codegen/lib-openapi.json` first
[group('server')]
[no-exit-message]
[working-directory(SERVER)]
generate-openapi:
    bash generate-openapi.sh

# Regenerate all client libraries
[no-exit-message]
codegen:
    {{ PYTHON }} regen_openapi.py

# Invokes `docker compose` with the testing configuration in the server directory
[group('server')]
[no-exit-message]
[working-directory(SERVER)]
dc *args='':
    docker compose -p svix-oss -f {{ SERVER / "testing-docker-compose.yml" }} {{ args }}
