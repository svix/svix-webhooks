Plan: Self-Hosted Svix — Remove Cloud Dependency
## Context
The goal is to run the Svix webhook server entirely on local/private infrastructure without depending on svix.com cloud. After investigation, the server is already self-hostable with no mandatory cloud dependencies. No code changes are required for basic operation. There are two minor optional cleanups that improve clarity.

## Findings
No mandatory cloud calls exist
No hardcoded calls to api.svix.com, play.svix.com, or any Svix cloud endpoint
No cloud API key or token required
All JWT signing is local
cfg.rs defaults region to "self_hosted" — indicating this is the intended deployment mode

## One optional cloud-touching feature: Operational Webhooks
File: server/svix-server/src/core/operational_webhooks.rs

This feature sends lifecycle events (endpoint created/disabled, message attempt exhausted, etc.) to a user-configured URL using the Svix SDK. It is fully optional — controlled by:


SVIX_OPERATIONAL_WEBHOOK_ADDRESS=<your-url>   # if unset, feature is disabled entirely
When operational_webhook_address is None, send_operational_webhook() returns early at line 137 with no outbound call.

Two unused config fields with cloud defaults
File: server/svix-server/src/cfg.rs — InternalConfig struct (lines 435–452)

app_portal_url — defaults to "https://app.svix.com" but is never read outside cfg.rs
region — defaults to "self_hosted" and is never read outside cfg.rs
These fields have no effect and require no changes.

## Changes Required
1. Configuration (required — no code changes)
Ensure .env / config.toml does NOT set SVIX_OPERATIONAL_WEBHOOK_ADDRESS. This is already the case in the current .env files.

File: server/.env and server/svix-server/.env

Confirm SVIX_OPERATIONAL_WEBHOOK_ADDRESS is absent (it is — nothing to change)
2. Optional: Change app_portal_url default to localhost (cosmetic)
File: server/svix-server/src/cfg.rs line 451

Change the default from "https://app.svix.com" to "http://localhost:8080" (or your own URL) to make it obvious the server is self-hosted. This field is currently unused so has no functional effect.


// Before
fn default_app_portal_url() -> String {
    "https://app.svix.com".to_owned()
}

// After
fn default_app_portal_url() -> String {
    "http://localhost:8080".to_owned()
}
Summary
The server is already fully self-hosted. To run it without any cloud dependency:

Start Docker containers: docker compose up -d (from server/)
Run migrations: cd server/svix-server && cargo run -- migrate
Start server: cargo run (from server/svix-server/)
Generate a token: cargo run jwt generate
Use the local API at http://localhost:8071 with that token
No calls to svix.com will ever be made as long as SVIX_OPERATIONAL_WEBHOOK_ADDRESS is not set.

Verification
Run cargo test --lib — all unit tests should pass
Run cargo test --test it e2e_message — integration tests use local server only
Optionally: run tcpview or netstat while the server is running to confirm no outbound connections to svix.com