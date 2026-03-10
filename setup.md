Setup Required
1. Install Rust (one-time)
Run this in a terminal (PowerShell or cmd):


winget install Rustlang.Rustup
Or download and run the installer from https://rustup.rs. After installing, restart your terminal so cargo and rustup are on your PATH.

Then install the required components:


rustup default stable
rustup component add clippy rustfmt
2. Install additional Rust tools

cargo install sqlx-cli cargo-nextest cargo-watch
3. Start the test databases
Docker is already installed and running. From the repo:


cd server
docker compose -f testing-docker-compose.yml up -d
This starts PostgreSQL, Redis, Redis Cluster, Redis Sentinel, and RabbitMQ — all needed for the full test suite.

4. Set up the .env file and run migrations

cd server/svix-server
cp development.env .env
Then run migrations (wait for DB to be ready first):


cd server
cargo run --no-default-features -- --wait-for 15 migrate
(old: cargo run -- --wait-for 15 migrate)

5. Run tests
Unit tests only (no Docker needed):


cd server
cargo test --lib
Full integration test suite:


cd server
./run-tests.sh
Or a specific test by name pattern:


cd server
cargo test --test '*app*'