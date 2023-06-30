# Changelog

## Next
* 

## Version 1.5.2
* Libs/JavaScript: Fix regression in previous release and allow passing `Buffer` to sign/verify.

## Version 1.5.1
* Server: Implement the send-example route.
* Libs/JavaScript: Add an explicit check that payload is a string.
* Libs: Fix a bug with integration key creation.

## Version 1.5.0
* Server: update OpenSSL dep and fix an incredibly slow memory leak.
* Libs/Go: support passing `WithContent` to `List Attepmted Messages`
* Libs/Python: fix regression in Python library (causing some functions not to work).

## Version 1.4.12
* Sync OSS package version with our internal version.
* Server: support comma separated array query params
* Bridge: initial release
* Libs: update OpenAPI spec
* Libs: add support for "Background Task" endpoints
* Libs: add support for since/until to endpoint stats
* Libs/Go: add missing `endpoint_id` option to list attempts by msg.
* Libs/JavaScript: add missing `withContent` field to `MessageListOptions`.
* Libs/Rust: fix typo in error handling message

## Version 0.85.1
* Libs: update OpenAPI spec

## Version 0.85.0
* Server: add a missing migration for operational webhooks event types.
* Libs/Ruby: fix the library failing to load due to missing dependency.
* Libs: **[Semi-breaking]** we changed the return value of the transformation simulation endpoint. It's technically a breaking page, though the API is private so it shouldn't affect people.

## Version 0.84.1
* Libs: regenerate libs from the correct openapi spec (could have caused potential authentication issues)

## Version 0.84.0
* Server: add (beta) RabbitMQ implementation
* Server: upgrade redis-rs and a few other dependencies
* Server: improve OpenAPI generation
* Server: fix issue with Redis being required in the docker image
* Libs/Rust: support choosing the wanted TLS implementation

## Version 0.83.1
* Libs: update OpenAPI spec

## Version 0.83.0
* Server: bump deps
* Libs: Add `send-example` wrapper to libraries
* Libs/Go: alias missing types from internal dir / openapi package

## Version 0.82.1
* Libs/Java: serialize nulls when making HTTP requests

## Version 0.82.0
* Server: refactor expired message cleaner to be incremental (prevent locks)
* Server: improve OpenAPI generation
* Server: Fix handling of very large numbers in json (larger than i64)
* Server: do not throw error on missing payload in worker
* Server: update dependencies

## Version 0.81.0
* Libs: add support for creating application when creating a message
* Libs/Go: bump Go version

## Version 0.80.0
* Libs: add `prev_iterator` and `order` support to application list
* Libs: add `prev_iterator` support to event type list
* Libs/C#: **[Breaking]** change default value for `SvixOptions.Throw` to `true`

## Version 0.79.0
* Server: support prev_iterator for application and event type listing
* Server: fix returning of 409 (CONFLICT) when inserting/patching an application with a conflicting `uid`
* Libs/Ruby: require (reexport) app portal models in ruby (fixing errors)
* Libs/C#: fix MessageAttempt querying when not filtering by status and code

## Version 0.78.0
* Server: add `order` query parameter for sorting endpoints
* Server: fix default sort order of endpoints to `desc` to match prod
* Libs: add support for `prev_iterator` for application and endpoints
* Libs/JS: fix sign function to support non-round dates
* Libs/Go: **[Breaking]** accept a context parameter in all Go lib methods

## Version 0.77.0
* Server: fix event_type array query parsing
* Server: fix bad `?channel=` queries
* Server: fire operational webhook on endpoint secret rotation
* Server: implement bidirectional pagination for endpoints
* Libs/Rust: glob-reexport all generated models in Rust

## Version 0.76.1
* Server: fix `/attempt/endpoint`'s broken `?channel=` query
* Libs/Rust: add missing exports to a few API endpoints
* Libs: fix naming of replay-missing methods in libraries (all but Rust)

## Version 0.76.0
* Server: add org_id and app_id to main tracing span
* Server: make `wait_for` timeout early and retry
* Server: add since/until query params to endpoint stats
* Server: add endpoints to expunge payload and response
* Server: clarify error message and documentation around filtered IP addresses
* Server: fix error message with endpoint filter types validation
* Server: fix `?after=<time>` ID Parsing in paginated endpoints
* Libs: add endpoints to expunge payload and response
* Libs: add replay missing messages functions
* Libs: add transformations APIs

## Version 0.75.0
* Server: add built in SSRF support (no need for an external proxy)
* Server: many worker improvements (see #704 for details)
* Server: fix Span HTTP Routes
* Server: update deps
* Server: add Length limit validation to EndpointHeader value
* Server: add missing root CAs to default docker image
* Server: add updated app-portal-access API endpoint (to replace the deprecated dashboard-access)
* Server: add feature flag gating to event types
* Libs/Rust: Enable TLS support in `reqwest` by default
* Libs: support the new feature flag gating of event types

## Version 0.74.1
* Libs/Rust: make all `*Options` and fields public (missing from previous release)

## Version 0.74.0
* Libs/Rust: make `ListOptions` and `PostOptions` fields public
* Libs/Rust: fix Webhook verification to accept &str

## Version 0.73.0
* Server: fix issue with potentially incorrect signatures for very small payloads.
* Server: fix Docker image to use `exec` so that svix-server will accept container process signals.

## Version 0.72.0
* Server: add metadata field to Endpoint endpoints
* Server: add missing msg id index causing some queries to be slow
* Server: make sensitive header detection in the `headers` API case insensitive
* Server: omit prevIterator from responses when `null`
* Server: fix string collation for some fields in the db (improves performance)
* Server: expose attempted URL on MessageAttemptOut
* Server: fix graceful server shutdown even when connection to queue is lost
* Server: fix listing and getting event-types permission regression (give app tokens access)
* Server: various code cleanups
* Server: update axum to 0.6
* JavaScript: make signature comparison constant time (thanks @arjunyel)

## Version 0.71.0
* Server: Add metadata to application CRUD
* Server: Update Rust, sea_orm, and other deps
* Libs: Update OpenAPI spec

## Version 0.70.0
* Server: create subcommand for wiping an organization's data
* Server: internal code improvements
* Libs/Java: fix issue with automatic region detection not working

## Version 0.69.0
* Server: ensure QueueTasks are deleted after acknowledgement
* Server: better handling of failures in streaming responses
* Server: update event-type schemas validation to be stricter
* Libs/Kotlin: support configuring retry schedule
* Libs: fix metadata field in Go and Python

## Version 0.68.1
* Server: update the event-type CRUD endpoints to be more strict about the schema type.
* Server: fix typo in error messages for unimplemented errors.
* Libs/Java: fix issue with creating multiple Svix instances in parallel.

## Version 0.68.0
* Server: add file/line information to errors for easier debugging.
* Server: update dependencies.
* Libs: add application/endpoint metadata fields.
* Libs/C#: Add netstandard2.0 support.
* Add gitleaks config to ignore test auth tokens.

## Version 0.67.0
* Server: include idempotency key in log spans.
* Server: correct trace ID handling in tracing spans.

## Version 0.66.0
* Server: add unique IDs when tracing worker tasks.
* Server: allow enabling extra tracing for db operations.
* Server: ensure tracing spans are recorded for all log levels.
* Server: reduce idempotency key lock time from 20s to 5s.
* Libs: expose endpoint stats function.

## Version 0.65.1
* Server: fix validation errors to be more informative.
* Server: more strict message payload validation to match the libs.
* Server: fix issue with messages being signed with expired keys in some situations (not a security concern, just superfluous data being sent).
* Server: attach a unique ID to a request (used in logs) when none is provided in headers.
* Libs/Python: fix typo in Python lib causing auto-detection of EU servers to fail.
* Libs/C#: make logger optional when creating SvixClient.

## Version 0.65.0
* Server: support "upsert" of entities on PUT methods.
* Server: support PATCH methods on entities for partial updates.
* Server: allow overriding redis_dsn for separate queue and cache DSNs.
* Libs: support "upsert" of entities on PUT methods.
* Libs/Ruby: fix region auto-detection.

## Version 0.64.2
* Server: change the dashboard authentication payload to note that the server is self hosted.
* Libs/JavaScript: fix issue when signing/verifying payloads with high Unicode codepoints (e.g. some new emoji)

## Version 0.64.1
* Libs/Go: export missing `svix.NullableString` utility.

## Version 0.64.0
* Server: disable endpoints on repeated failures.
* Server: add retry functionality to the Redis queue (to be more resistant minor networking hiccups).
* Server: improve the delayed queue processing to be resilient to errors and better support concurrent processing.
* Server: gracefully shutdown the server on SIGTERM.
* Libs/Kotlin: fix compilation issues.
* Libs: automatically detect region (and URL) from auth token.

## Version 0.63.1
* Lib/JavaScript: fix setting string webhook secrets.

## Version 0.63.0
* Server: add support for encrypting webhook secrets in the database.
* Server: include the error message in the attempt's response for non HTTP errors.
* Server: change the CORS headers to be more strictly compliant.
* Server: wait-for db before attempting to run migrations.
* Server: add retry functionality to Redis cache.
* Libs: update libraries to accept a raw webhook secret.

## Version 0.62.1
* Libs: fix Kotlin and Java build

## Version 0.62.0
* Server: add support for asymmetric signatures.
* Server: ensure msg content exists before attempting resend.
* Server: Improve HTTP error response logging.
* Server: fix docker builds due to changes to Redis SSL.
* Libs: fix nullable fields to be marked as such (fixes parsing errors in some clients).

## Version 0.61.0
* Server: add OpenTelemetry support
* Server: send operational ("incoming") webhooks about events happening on the server.
* Server: enable TLS support for Redis
* Server: use correct timestamp for attempt's id and created.
* Server: add jitter to message retry times.
* Server: fix endpoint spelling in some error messages and comments.
* Server: add built in support for waiting for dependencies (database, redis) to be ready (instead of using a script).

## Version 0.60.0
* Server: normalize health status output to be in lowercase.
* Server: implement application portal endpoint and limited access tokens for it.
* Server: add endpoint stats endpoint for getting endpoint statistics.
* Server: add support for JSON log format for use in cloud environments.
* Server: add configuration options for the max pool size for DB and Redis connection pools.
* Server: add CORS headers and support pre-flight requests for use from the browser.
* Server: add a configuration option to only allow https endpoint URLs.
* Server: support passing an org_id when generating JWTs (`svix-server jwt generate`).
* Lib/Python: update httpx dependency to the latest version.
* Lib/Rust: fix mixup with validation errors marked as HTTP errors and vice-versa.

## Version 0.59.1
* Libs/Csharp: expose missing getters which prevented a lot of the lib's functionality to be used.

## Version 0.59.0
* Server: make the `health` endpoint more useful by adding more status checks to it.
* Server: parse `retry_schedule` as an array rather than string.
  * People should use the next syntax: `[1, 2]`, though the old syntax is still supported for backwards compatibility.
* Server: add validation to ensure endpoint URL scheme is either http or https.
* Libs/Python: show a more useful error for obviously malformed secrets.
* Libs/JavaScript: show a more useful error for obviously malformed secrets.

## Version 0.58.2
* Libs and server: update OpenAPI spec

## Version 0.58.1
* Server: Fix logging configuration in production builds to actually be respected.
* Server: improve how we run Redis migrations.

## Version 0.58.0
* Lib/Rust: add a Rust API client + webhook verification library!
* Lib/Python: fix package installation on Windows.
* Lib/Csharp: make some parameters optional for better ergonomics.
* Server: remove `updated_at` field from static entities.
* Server: change to a more efficient first message dispatch.
* Server: change the redis queue to use redis streams.
* Server: serve nice API docs under `/docs`.
* Server: start version tagging the Docker images. (`latest`, `x`, and `x.y`).
* Server: allow PATCH to delete header values by passing null.
* Server: make it possible to change the hard pagination limits to be soft.
* Server: make the redis queue implementation automatically recover from redis crashes.
* Server: add a `--run-migrations` flag to automatically run migrations (flag still passed in Docker).
* Server: fix worker to not follow HTTP redirects when calling webhooks.

## Version 0.57.2
* Server: add support for get-or-create when creating applications.
* C#: fix library compilation (broke in the previous release.

## Version 0.57.1
* Libs: fix all libraries to handle 429 (rate limiting).
* Server: update docker image to use Rust 1.60
* Server: preserve header name capitalization for custom endpoint headers.
* Server: gracefully handle non-textual webhook responses (so endpoints returning non strings).
* Server: delete message content after the specified retention period.

## Version 0.57.0
* Server: add support for idempotency.
* Server: add prev_iterator support to endpoints that should support it.
* Server: make parameter validation stricter in various places.
* Server: improve error messages for bad configurations.
* Server: support for Redis clusters.
* Server: add a memory cache backend.
* Server: fix health endpoint to return an empty response to match its status code (204).
* Server: fix issue that can cause messages to be sent more than once when clients timeout.
* Server: fix support for endpoint id in the list attempted messages and list attempted destinations APIs.
* C#: add C# API client library.

## Version 0.56.0
* Libs/Python: fix user agent to actually work.
* Libs/Python: increase read timeout.
* Server: extend prev_iterator implementation to allow for before and after.
* Server: update Axum dep to latest version.
* Server: add more tests to the suite.

## Version 0.55.0
* Server: fix marking of some HTTP errors as failed.
* Server: change base docker image to debian-slim
* Server: update deps.
* Server: added missing list attempts endpoints and missing query parameters to some endpoints.
* Server: switch to KsuidMs for extra precision.
* Server: add missing validation for `IdOrUid` so that they return 422.

## Version 0.54.2
* Server: fix release CI round 2

## Version 0.54.1
* Server: fix release CI

## Version 0.54.0
* Libs: add retry and request-id headers for easier debugging.
* Server: add more release targets (macOS and aarch64).
* Server: make 422 errors compatible with the Svix service.

## Version 0.53.2
* Ruby: Fix CI to not include Vendor data in package.

## Version 0.53.1
* Java & Kotlin: Build against java 11
* Ruby: Fix a broken import (we changed the name of a webhook event) 🐞

## Version 0.53.0
* Python: **Breaking** The python library is now fully typed!  There may be some breaking changes releated to this upgrade (including dropping support for Python 2.X). Please check test and check your integration before upgrading to this version. 🤓
* Python: New Async API via SvixAsync! 🚀
* Libs: Remove the `prevIterator` option from message attempt options (This was added by mistake, never worked and was never meant to work. All iterators should be passed via the `iterator` option. Sorry for the confusion!)
* Libs: Allow filtering by messages by channel

## Version 0.52.0
* Libs/JS: Fix for setting Idempotency-Key when one isn't set  🐞
* Libs/All: Add `msg_id` to attempt list responses (`MessageAttemptOut`)
* Libs/All: New GetOrCreate application function 🆕
* Server: Remove deprecated axum API usage
* Server: Implement PATCH for endpoint headers API 🆕
* Server: Add support for an endpoint cache for improved performance when sending 🚀

## Version 0.51.0
* Libs: Support for passing an idempotency key to post commands
* Libs: Add support for filtering by channel
* Kotlin & Go: Support filtering by event type
* JS: fix for API calls using a default idempotency key when one isn't set 🐞

## Version 0.50.0
* Server: Set custom endpoint headers when sending via worker
* Libs: Add support for filtering by StatusCodeClass in attempts API

## Version 0.49.0
*  Libs: Add support for reverse iteration (prev_iterator) & after param in list commands ⬅️
*  Server: Add support for adding & managing custom Endpoint Headers 🆕

## Version 0.48.0
* Libs: Support for new `/attempt/` API via list_for_msg and list_for_endpoint. 🚀
* **Deprecation warning:** `message_attempt.list` is deprecated in favor of this new API. ❌

## Version 0.47.1
* CI: Autorelease server artifacts on github releases 🆕

## Version 0.47.0
* **Initial OSS release of Svix Server!** 🚀
* Java & Kotlin: Minor binding fixes to bring them up to date with other libs 🐞

## Version 0.46.0
* Go: **Breaking**, Changed module name to `github.com/svix/svix-webhooks` 💥
  (We're sorry for any inconvenience this may cause you.)
* Python: Add support for passing a custom `idempotency-key` for POST actions 🔂
* All: Add support for filtering by `channel` for message list APIs 🆕

## Version 0.45.0
* Python: Add new get_or_create function for application API 🆕

## Version 0.44.0
* All: Add support for the new Integration API 🆕

## Version 0.43.2
* JS: Bump vulnerable dependency 👮
* Ruby: Cleanup gem level imports 💅

## Version 0.43.1
* Java & Kotlin: Minor internal fix to restore version number parity with other libs

## Version 0.43.0
* Python: Allow explicit `None` to be passed for optional fields ❌

## Version 0.42.3
* JS: Use fetch fork from npm to avoid cache/name clashes 🐞

## Version 0.42.2
* JS: Rename dependency to avoid name-clashes 🐞

## Version 0.42.1
* All: Update Schema

## Version 0.42.0
* All: Support new Channels API 📺

## Version 0.41.2
* Javascript: Enable keepalives for improved performance 🚀

## Version 0.41.1
* Kotlin: Ensure response bodies are properly closed 🐞

## Version 0.41.0
* All: Add new `ServerUrl` option for overriding the APIs base URL (Useful for testing) 🆕

## Version 0.40.0
* All: Support for `payload_retention_period` in message API. 🆕

## Version 0.39.0
* Ruby: Upgrade to bundler 2 🆕
* Ruby: Fix endpoint.update parameters 🐞 (thanks [sevensidedmarble](https://github.com/sevensidedmarble)!)
* Go: Add ability to verify webhook payloads while ignoring timestamp enforcement ❌🕝
* Go: Regenerate OpenAPI Spec

## Version 0.38.0
* All: Add support for retrieving a single event type 🆕
* Javascript: Add readme for npm 📚
* Ruby: Fix base64 import on case-sensitive file systems 🐞  (thanks [mikelarkin](https://github.com/mikelarkin)!)

## Version 0.37.0
* All: Add support for custom endpoint header API 🆕

## Version 0.36.0
* All: Add endpoint recover function ❤️‍🩹

## Version 0.35.0
* All: Add updatedAt fields to Application, Endpoint & EventType 🆕

## Version 0.34.2
* All: Better client-side handling of rotateSecret errors 🆕

## Version 0.34.1
* Ruby: Fix endpoint list function input 🐞

## Version 0.34.0
* All: Add include_archived option to event_type API ➕
* All: Support secret rotation for endpoints 🔁
* Kolin: Fix getSecret return type 🐞

## Version 0.33.0
* PHP: Fix 5.6 compatibility issue 🐞

## Version 0.32.0
* Ruby: Initial API bindings! 🚀
* Kotlin: Add client-side retries 🔁

## Version 0.31.0
* Python: Add additional message-attempt APIs. 🆕

## Version 0.30.0
* Kotlin: Initial async kotlin library 🚀
* All: Improved naming for Integer Enums 📛
* JS: Update dependencies 🆕
* Java: Cleanup list option code using generics 🤓

## Version 0.29.0
* All: Add before option to message APIs
* Java: Cleanup list options
* All: Add ability to set signing secret
* All: add event type filtering to message attempt API
* All: support for endpoint uids

## Version 0.28.0
* All: Adds support for unbranded webhooks 📋
* PHP: Improve tests and validation for sign function 🐞
* Java: Set descriptive message on APIExceptions 🆕

## Version 0.27.0
* All: Adds Sending to MessageStatus 📤
* All: Add support for Schemas in EventType API 🆕

## Version 0.26.0
* All: Some of the list options structs have been renamed (this is a minor breaking change in some languages) 💥
* All: Filter messages by EventType ⏳
* All: Add custom user-agents 🧑‍💻
* All: Retry 5XXs & network errors 🔁
* All: Add nextAttempt to MessageEndpointOut 🆕
* All: Update Schema 🆕
* Javascript: Update Dependencies 🆕
* Ruby: Update Dependencies 🆕

## Version 0.25.0
* All: Add nextAttempt to EndpointMessageOut 🕛

## Version 0.24.0
* Java: Bump dependencies 🆕
* All: Add support for rate limiting 📈

## Version 0.23.0
* All: Bump openapi-generator to 5.2.0, fixes bug in python lib which can throw if additionalProperties are found 🐛

## Version 0.22.0
* Python: Cleanup explicit exports in python 🤓
* All: Add Disabled flag to endpoint models ❌

## Version 0.21.0
* All: Add Sign functions for generating webhook signatures 🔏
* Python: Enable typing 🤓
* Csharp: Initial webhook verification library 🚀
* Go: Set default http client timeout and allow overriding of client. ⏲️
* All: Bump openapi spec, fixes bug in resend function, which could throw an error in some languages on return. 🐛

## Version 0.20.1
* JavaScript, Python and Go: fix regression with server URLs not being set correctly.

## Version 0.20.0
* All: Update OpenAPI spec
* Javascript: Fix bug with message attempt resend (would throw on return)
* Python: Fix optional bug where optional return types would throw

## Version 0.19.1
* Python: Fix issue with pip package not generating the `openapi_client` properly on deploy

## Version 0.19.0
* Python: Clean up exports, add API exceptions as `svix.exceptions`

## Version 0.18.0
* All: Enforce timestamp tolerance when verifying
* All: Support new webhook signature prefixes when verifying
* Go: Expose new error class for checking API errors
* PHP: Fix issue with psr-4 autoloading
* Python: Properly export WebhookVerificationError
* Python: Fix bug in event_type create function

## Version 0.17.0
* Migrate to new GitHub org `svix`! :rocket:

## Version 0.16.0
* Update the OpenAPI spec and change structures accordingly

## Version 0.15.0
* go: Add DebugURL option for overriding API URL
* Ruby: initial release
* PHP: initial release

## Version 0.14.0
* Update the Message model to call the payload `payload` instead of `data`.
* Add List event_types to go library
* Fix bug in endpoint create function

## Version 0.13.0
* Release Go lib
* Add function to update event_types.

## Version 0.12.4
* Update OpenAPI spec to the latest version

## Version 0.12.3
* Rename to Svix
* Sync library versions

## Version 0.0.0 (Initial open-sourcing)
* Make the library open-source
* Future releases will have proper changelogs and a synchronised version scheme.
  * Just waiting on the rename of the library
