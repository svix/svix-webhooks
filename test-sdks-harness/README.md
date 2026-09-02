# AutoConfig SDK live harness

Runs the same four AutoConfig scenarios against a live Svix server in every SDK that has the API.

v2 tokens bind once. Re-run setup (or `./run.sh` without `--skip-setup`) before a second full pass.

## One command

```bash
export SVIX_TOKEN='testsk_....'          # org / management token, not auto_v*
export SVIX_SERVER_URL='http://localhost:8071'  # optional, this is the default
./test-sdks-harness/run.sh
```

`run.sh` mints a fresh app and five tokens per language, writes `fixtures.json` (gitignored), then runs A–E. Scenario E binds via `subscribe` on one `AutoConfigConsumer`, then receive/commit on a second instance of the same token.

```bash
./test-sdks-harness/run.sh --skip-setup csharp ruby
./test-sdks-harness/run.sh --setup-only
LANGS=javascript,python ./test-sdks-harness/run.sh
```

## Scenarios

| | What | Token | API |
|---|---|---|---|
| A | HTTP subscribe | `auto_v1_` | `AutoConfig.subscribe` → `EndpointOut` |
| B | HTTP subscribe | `auto_v2_` | same constructor, v2 route |
| C | Poller subscribe, send, receive, commit | `auto_v1_` | `AutoConfigConsumer` |
| D | Same poller path | `auto_v2_` | native `DestinationOut` |
| E | Receive + commit on a fresh instance | `auto_v2_` | `subscribe` on instance 1, GET + poller on instance 2 |

HTTP subscribe always sets `eventTypes: [it.autoconfig.ping]`. This org 422s without a filter. Poller `receive` uses `startingPosition=earliest` and `leaseDurationMs=2000`, 10 tries, 2s apart.

The org token is used only for setup and `POST /msg/`. SDKs authenticate with `tok` from the magic token. Do not send the `auto_v*` string as `Authorization`.

## Server notes

Health must be 204 on `SVIX_SERVER_URL`. Tokens from a local server often embed `surl: http://localhost:8040`. If that proxy is 502, setup rewrites `surl` to `SVIX_SERVER_URL` and re-encodes with **padded** standard base64. Do not strip `=`.

Polling needs Diom. Setup aborts with `DIOM_MISSING` if the smoke poll returns 501.

## Toolchain (this machine)

| Lang | How the runner invokes it |
|---|---|
| JavaScript | `npx tsx` from `javascript/`, imports `src` (not `dist`) |
| Python | `uv run python` in `python/` |
| Go | `go run` with `replace` → repo root |
| Rust | `cargo run` path-dep on `rust/` |
| Java / Kotlin | JDK 17 (`JAVA_HOME` or `/usr/libexec/java_home -v 17`). Newer JDKs break Kotlin. |
| C# | `~/.dotnet/dotnet` (net8), not the Homebrew 7.0 `dotnet` |
| Ruby | `RUBYLIB=ruby/lib`; gems from `~/.gem/ruby/4.0.0` if present |
| PHP | local `php` or `docker run --network host php:8.3-cli` + repo `vendor/` |

## Results

Each runner prints:

```
LANG: <lang>
A: PASS|FAIL — ...
B: PASS|FAIL — ...
C: PASS|FAIL|BLOCKED — ...
D: PASS|FAIL|BLOCKED — ...
Notes: ...
```

Last live run and the remaining SDK/codegen bugs: [FAILURES.md](FAILURES.md).
