# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a monorepo for Svix - a webhooks-as-a-service platform. It contains:

- **`server/`** - The core Svix server (Rust, Axum, SeaORM, PostgreSQL, Redis)
- **`svix-cli/`** - CLI tool for interacting with the Svix API (Rust)
- **`bridge/`** - Agent to bridge message queues (Kafka, RabbitMQ, SQS, GCP Pub/Sub, Redis) with Svix webhooks (Rust)
- **`codegen/`** - OpenAPI-based SDK code generation tooling
- **Client SDKs**: `go/`, `python/`, `javascript/`, `java/`, `kotlin/`, `ruby/`, `csharp/`, `rust/`, `php/`

The client SDKs are **generated** from `lib-openapi.json` via `./regen_openapi.py` using a Docker/Podman-based codegen tool.

## Server Development (`server/`)

### Setup

```bash
cd server/svix-server
cp development.env .env
docker compose up  # starts PostgreSQL and Redis
cargo sqlx migrate run  # run migrations
```

### Running

```bash
cd server/svix-server
cargo watch -x run  # auto-reloading dev server
cargo run jwt generate  # generate an auth token
```

### Linting

```bash
cd server
cargo clippy --fix
cargo fmt
```

### Testing

Integration tests require running PostgreSQL and Redis (start with `docker compose -f testing-docker-compose.yml up -d`), then run migrations:

```bash
cd server
cargo run -- --wait-for 15 migrate
```

Run all tests across all queue/cache configurations:
```bash
cd server
./run-tests.sh
```

Run only unit tests (no external dependencies):
```bash
cd server
cargo test --lib
```

Run a specific test by name pattern:
```bash
cd server
cargo test --test '*app*'
```

The full test suite runs 7 configurations covering Redis, RedisCluster, RedisSentinel, RabbitMQ, and in-memory queue backends.

### Database Migrations

```bash
cd server/svix-server
cargo sqlx migrate add -r MIGRATION_NAME  # create new migration
cargo sqlx migrate run                     # apply migrations
cargo sqlx migrate revert                  # revert latest migration
cargo sqlx migrate info                    # view status
```

### OpenAPI

```bash
cd server
./generate-openapi.sh         # regenerate openapi.json
./generate-openapi.sh --check  # verify no uncommitted changes (used in CI)
```

## CLI Development (`svix-cli/`)

The CLI's API command code is **generated** from `lib-openapi.json`. After updating the OpenAPI spec, run `./regen_openapi.py` from the repo root before working on the CLI.

```bash
cd svix-cli
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo fmt
cargo nextest run  # or cargo test
```

## Bridge Development (`bridge/`)

```bash
cd bridge
cargo build
cargo test
```

## SDK Code Generation

All client SDK code in `go/`, `python/`, `javascript/`, `java/`, `kotlin/`, `ruby/`, `csharp/`, `rust/`, `php/`, and `svix-cli/src/cmds/api/` is **generated** and should not be edited directly.

To regenerate after modifying `lib-openapi.json` or templates in `codegen/templates/`:

```bash
./regen_openapi.py  # requires Python 3.11+ and Docker or Podman
```

The codegen configuration is in `codegen/codegen.toml`. Templates use Jinja2 and the `openapi-codegen` Docker image.

## Server Architecture

The server is built on **Axum** with **SeaORM** for database access. The main entry points:

- `server/svix-server/src/lib.rs` - `run()` function that wires up all components
- `server/svix-server/src/v1/mod.rs` - API router assembling all endpoint modules
- `server/svix-server/src/worker.rs` - Background queue worker that dispatches webhooks
- `server/svix-server/src/cfg.rs` - Configuration loading (from `config.toml`, `.env`, or env vars prefixed `SVIX_`)

Key modules:
- `v1/endpoints/` - HTTP handlers for application, endpoint, message, attempt, event_type, auth, admin
- `db/models/` - SeaORM entities (application, endpoint, message, messageattempt, messagedestination, etc.)
- `core/` - Shared logic: cache (Redis/memory/none), cryptography, idempotency, security (JWT), operational webhooks
- `queue/` - Task queue abstraction over Redis, RabbitMQ, and in-memory backends
- `redis/` - Redis connection management (standalone, cluster, sentinel)

`AppState` (in `lib.rs`) holds the shared database connection, queue producer, config, cache, and operational webhook sender - injected into all Axum handlers.

The server can run API and worker roles independently (controlled by `api_enabled`/`worker_enabled` config flags), supporting split deployments.

## Configuration

Config is loaded via `figment` from (in priority order): environment variables (`SVIX_` prefix) > `.env` file > `config.toml`. See `server/svix-server/config.default.toml` for all options.

Key config values: `jwt_secret`, `db_dsn` (PostgreSQL), `redis_dsn`, `queue_type` (redis/rediscluster/redissentinel/rabbitmq/memory), `cache_type`.

## CI Notes

- MSRV is **1.88.0** for both server and CLI
- Server CI uses `cargo nextest` when available
- Formatting uses nightly rustfmt (`cargo fmt` with nightly toolchain)
- CI checks that OpenAPI spec matches generated output (run `./generate-openapi.sh --check`)

See plan.md for the current project roadmap.