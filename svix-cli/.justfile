set dotenv-load
set default-list

import "../.justfile-common-rust"

HERE := justfile_directory()

# Run the CLI
run *args='':
    cargo run -- {{ args }}
