set dotenv-load
set default-list

import "../.justfile-common-rust"

HERE := justfile_directory()

# Run the bridge server
run *args='':
    cargo run -- {{ args }}

# Invokes `docker compose` with the testing configuration in the 'bridge' directory
[no-exit-message]
dc *args='':
    docker compose -p svix-oss -f {{ HERE / "testing-docker-compose.yml" }} {{ args }}
