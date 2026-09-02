# AutoConfig live-test failures (2026-09-10)

Last full rerun used padded tokens and `eventTypes` on every HTTP subscribe. C#, Ruby, and Kotlin now accept a missing `DestinationOut.config`. Kotlin sends `Content-Type: application/json`. Java struct-enum models omit null optionals.

## Scoreboard

| Lang | A v1 HTTP | B v2 HTTP | C v1 poller | D v2 poller | E v2 existing |
|---|---|---|---|---|---|
| JavaScript | PASS | PASS | PASS | PASS | PASS |
| Python | PASS | PASS | PASS | PASS | PASS |
| Go | PASS | PASS | PASS | PASS | PASS |
| Rust | PASS | PASS | PASS | PASS | PASS |
| Java | PASS | PASS | PASS | PASS | PASS |
| Kotlin | PASS | PASS | PASS | PASS | PASS |
| C# | PASS | PASS | PASS | PASS | PASS |
| Ruby | PASS | PASS | PASS | PASS | PASS |
| PHP | PASS | PASS | blocked | blocked | blocked |

## Still broken

### PHP `oneOf` models are empty stubs

After a PHP codegen rerun, these files are still only the `@generated` header:

- `php/src/Models/DestinationIn.php`
- `php/src/Models/DestinationOut.php`
- `php/src/Models/AutoConfigSinkType.php`
- `php/src/Models/StreamSinkIn.php` (same pattern, unused here)

`AutoConfigConsumer` already calls `DestinationIn` / `DestinationOut::fromMixed` / `AutoConfigSinkType::create`. There is no class to run. A/B only need `EndpointIn`, which generates fine.

Do not hand-fill the stubs. Fix the PHP generator for tagged unions, regenerate, then rerun C/D/E.

## Fixed since the last write-up

- **C# / Ruby / Kotlin D** — missing `DestinationOut.config` now deserializes as `{}`.
- **Kotlin C** — `SvixHttpClient` sets `Content-Type: application/json` on request bodies.
- **Java D/E** — struct-enum surrogates use Jackson `NON_NULL`, so unset `batchSize` / `maxWaitSecs` are omitted instead of `null` (`expected u16`).

## Not SDK bugs

These showed up in the first pass and are easy to reintroduce.

Missing `eventTypes` on `EndpointIn` 422s on this org (`filterTypes` required). The harness always sets `eventTypes: [it.autoconfig.ping]`.

Tokens from a local server often embed `surl: http://localhost:8040`. That proxy was 502; the API process is `:8071`. Setup rewrites `surl` and keeps standard padding.

An earlier fixture rewrite used `.rstrip("=")` after changing `surl`. Go, Rust, and C# rejected `len % 4 == 2` payloads. JS and Python tolerated it. Real server tokens from `STANDARD.encode` are padded. Do not strip `=` in setup.

V2 subscribe binds once. A second HTTP or poller subscribe on the same token is 409 `already_bound`. Mint new tokens (rerun setup).

501 on poll means Diom is off. Infra, not an SDK bug.

## Suggested fix order

1. PHP `oneOf` models, then rerun C/D/E.
