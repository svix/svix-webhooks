<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![Build Status](https://github.com/svix/svix-webhooks/workflows/Server%20CI/badge.svg)](https://github.com/svix/svix-webhooks/actions)
[![Server Security](https://github.com/svix/svix-webhooks/actions/workflows/server-security.yml/badge.svg)](https://github.com/svix/svix-webhooks/actions/workflows/server-security.yml)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

## Svix is the enterprise ready webhook service

Svix makes it easy for developers to send webhooks. Developers make one API call, and Svix takes care of deliverability, retries, security, and more. For more information, please refer to the [Svix homepage](https://www.svix.com).

# Running the server

For information on how to use this server please refer to the [running the server](../README.md#running-the-server) in the main README.

# Building from source

You would need a working Rust complier in order to build Svix server.
The easiest way is to use [rustup](https://rustup.rs/).
Make sure you have a working Rust compiled (e.g. by using [rustup](https://rustup.rs/)).

Once rustup is installed, switch to the `stable` toolchain and install required components:
```sh
rustup default stable
rustup component add clippy rust-src cargo rustfmt
```

Also build additional rust dependencies:
```sh
cargo install sqlx-cli cargo-watch
```
(`cargo-watch` is used for automatic reload while developing and can be skipped)

Finally, clone and build Svix:

```sh
git clone https://github.com/svix/svix-webhooks
cd svix-webhooks/server/
cargo install --path svix-server
```

# Development

## Run the development server

Svix needs a few ancillary services.
```sh
ln -s docker-compose.base.yml docker-compose.yml
docker compose up -d
```

Setting some server configuration:
```sh
cd svix-server/
cp development.env .env
```

### Generating an auth token

Now generate an auth token, you can do it by running:
```sh
cargo run jwt generate
```
This is the `Bearer` token that you can use to authenticate requests.

See [the main README](../README.md) for instructions on how to generate the auth token in production.

### Run the SQL migrations

One last missing piece to the puzzle is running the SQL migrations.

From the same directory as the `.env` file run:
```sh
cargo sqlx migrate run
```

More useful commands:
```sh
# View the migrations and their status
cargo sqlx migrate info
# Reverting the latest migration
cargo sqlx migrate revert
```

### Starting Svix

To run the auto-reloading development server run:
```sh
cargo watch -x run
```

Test the server:
```sh
curl -X 'GET' \
  'http://localhost:8071/api/v1/app/' \
  -H 'Authorization: Bearer <what you generated before>' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json'
```

## Creating new SQL migration

As you saw before you run/revert migrations. To generate new migrations you just run:
```sh
cargo sqlx migrate add -r MIGRATION_NAME
```

And fill up the created migration files.


## Linting

Please run these two commands before pushing code:

```sh
cargo clippy --fix
cargo fmt
```

## Testing

By default, `cargo test` will run the full test suite which assumes a running PostgreSQL and Redis database.
These databases are configured with the same environment variables as with running the actual server.

The easiest way to get these tests to pass is to:
    1. Start background services using `docker compose -f docker-compose.testing.yml up`
    2. Create a `.env` file as you would when running the server for real.
    3. Migrate the database with `cargo run -- migrate`.
    4. Run `cargo test --all-targets`

Alternatively, if you're only interested in running unit tests, you can just run `cargo test --lib`. These tests don't make any assumptions about the surrounding environment.

To run only a specific test (e.g. only the application tests), you can use the `--test` flag to `cargo test` which supports common Unix glob patterns. For example: `cargo test --test '*app*'`.
