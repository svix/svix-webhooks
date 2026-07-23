# Changelog

## Unreleased

## Version 1.99.0
* Fix warning when opentelemetry is disabled

## Version 1.98.0
* Set OCI metadata on Docker images
* Embed package metadata in compiled binaries on Linux
* Improve validation of endpoint URLs (SVIXSEC-2026-0001)

## Version 1.97.0
* Add `statusText` to `EndpointMessageOut` (the response type for `v1.message-attempt.list-attempted-messages`), matching the cloud version
* Add `statusText` to `MessageEndpointOut` (the response type for `v1.message-attempt.list-attempted-destinations`), matching the cloud version
* Add `canceled` to `EndpointStatsOut`, matching the cloud version. This is always 0 in the OSS server, which doesn't currently track message cancellations
* Add `updatedAt` to `RecoverOut` (the response type for `v1.endpoint.recover`), matching the cloud version. Note that EE does not support incrementally checking background job status, so this always contains the timestamp at which the job was created
* Add `gracePeriodSeconds` to `EndpointSecretRotateIn`, allowing users to customize how long the old key is still valid for (in a range from 0, which means immediate expiry, to 7 days)
* Update dependencies

## Note: missing versions

Before v1.100.0 of svix-server, server versions were synced with SDK versions and we published
releases even when the server component had no changes since the prior release.
That is why version numbers are not contiguous from here on.

## Version 1.93.0
* Warn when Redis pending timeout is shorter than worker request timeout + buffer (thanks [@vinay0826])
* Add missing messageattempt index (as a new kind of background migration)

## Version 1.91.0
* Remove old endpoint secrets on rotation

## Version 1.90.0
* Add `throttleRate` to endpoint and application
  * This replaces `rateLimit` because the name caused some confusion
    (but is kept as a deprecated field for backwards compatibility)

## Version 1.89.0
* Add a prune helper command to delete old data
* Invalidate CMA cache when creating/updating endpoints or apps
* Return empty JSON object from v1.message-attempt.resend
* Build Docker image with Rust 1.94

## Version 1.87.0
* Respect `retry-after` header on error responses (within limits) (thanks [@vinay0826]!)

[@vinay0826]: https://github.com/vinay0826

## Version 1.86.0
* Fix non-determinism with regards to overrides of specific header names
* Bump MSRV to 1.88.0

## Version 1.83.0
* Add `status_text` field to `MessageAttemptOut` to match Svix Cloud
  * Makes current versions of the SDKs work with endpoints using this type again

## Version 1.82.0
* Allow specifying queue prefix (thanks [@turip]!)

[@turip]: https://github.com/turip

## Version 1.79.0
* Add support for creating applications in the `POST /api/v1/app/{app_id}/msg` and the `POST /api/v1/auth/app-portal-access/{app_id}` API calls

## Version 1.77.0
* DB refactor, improves performance of a few API endpoints

## Version 1.75.0
* Modify Dockerfiles to use cache mounts for improved build time; these now require Docker 1.2 or later to build

## Version 1.74.0
* Upgrade some core dependencies

## Version 1.72.0
* Upgrade Docker base image to Debian Trixie

## Version 1.69.0
* Reduce allocations (thanks [@fluiderson]!)
* Add healthcheck command (thanks [@y-nk]!)

[@fluiderson]: https://github.com/fluiderson
[@y-nk]: https://github.com/y-nk

## Version 1.68.0
* Ensure messagecontent expiration is set

## Version 1.66.0
* Support sending raw (pre-formatted) JSON payloads

## Version 1.65.0
* Add URL validation to operational server webhooks by @CodeMan62 in https://github.com/svix/svix-webhooks/pull/1887

## Version 1.64.1
* Add response duration tracking to webhook message attempts by @CodeMan62 in https://github.com/svix/svix-webhooks/pull/1877

## Version 1.45.0
* Fix Dockerfile exposed port to be 8071

## Version 1.42.0
* Return 413 on large payloads

## Version 1.41.0
* Improve error messages for configuration loading

## Version 1.40.0
* exit early if endpoints don't exist by @jaymell in https://github.com/svix/svix-webhooks/pull/1515

## Version 1.39.0
* Add Redis sentinel support
* Add OTEL metrics for Redis queues
* Add Redis DLQ support
* Several dependency upgrades and CI improvements

## Version 1.38.0
* Increase max endpoint rotations
* Return 409 on duplicate message insert
* Add `expunge` parameter to event-type deletion endpoint
* Add `message.attempt.recovered` operational webhook

## Version 1.33.0
* Add support for deprecating event types
* Add support for HTTP(S) proxies

## Version 1.32.0
* Fix parsing proxy config from process environment

## Version 1.30.0
* Support filtering by before and after at the same time

## Version 1.29.0
* Add support for SOCKS5 proxies

## Version 1.26.0
* add configuration to for changing service name on OpenTelemetry

## Version 1.25.0
* Enable redis `tcp_nodelay`
* Improve database error classification
* Update dependencies

## Version 1.24.0
* Update redis health check
* Clean up tracing spans for HTTP requests

## Version 1.22.0
* Don't require trailing slash at the end of request paths
* Improve testing performance and reduce redundant test runs
* Improve performance of `/api/v1/openapi.json` route
* Upgrade dependencies and improve code formatting
* Upgrade to Docker Compose v2

## Version 1.21.0
* Improve error information on failed assertions in Redis module
* Use omniqueue for Redis queue implementation
* Upgrade OpenTelemetry dependencies

## Version 1.20.0
* Performance improvements in test setup

## Version 1.18.0
* upgrade dependencies
* adopt omniqueue as a queue backend

## Version 1.17.0
* Upgrade hyper to 0.14.28

## Version 1.16.0
* Add `tag` parameter to list-message for Go, JavaScript, and Python.
* improvements to the expired message cleaner.

## Version 1.14.0
* separate out the message content to its own model.

## Version 1.13.0
* Fix tracking of backtrace in some error cases.

## Version 1.11.0
* update Docker image to Debian bookworm.
* update dependencies.

## Version 1.10.0
* fix docker image.
* release arm64 version!

## Version 1.9.0
* add missing field documentation for query params.
* warn people trying to use the JWT secret as an auth token.
* allow disabling TLS verification.
* use jemalloc as the global allocator.
* add Sentry support.
* update dependencies.
* update Docker base image and rust version.

## Version 1.8.1
* correctly disconnect the tracing provider when shutting down in some rare scenarios.

## Version 1.8.0
* Add `with_content` to attempt listing endpoints

## Version 1.7.0
* update Docker image to use latest Rust version
* support additional JWT signing algorithms (including asymmetric!).
* Avoid panics when using the wait-for option (better errors printing).

## Version 1.6.0
* add 'event_types' query param to 'list-attempted-messages'

## Version 1.5.1
* Implement the send-example route.

## Version 1.5.0
* update OpenSSL dep and fix an incredibly slow memory leak.

## Version 1.4.12
* Sync OSS package version with our internal version.
* support comma separated array query params

## Version 0.85.0
* add a missing migration for operational webhooks event types.

## Version 0.84.0
* add (beta) RabbitMQ implementation
* upgrade redis-rs and a few other dependencies
* improve OpenAPI generation
* fix issue with Redis being required in the docker image

## Version 0.83.0
* bump deps

## Version 0.82.0
* refactor expired message cleaner to be incremental (prevent locks)
* improve OpenAPI generation
* Fix handling of very large numbers in json (larger than i64)
* do not throw error on missing payload in worker
* update dependencies

## Version 0.79.0
* support prev_iterator for application and event type listing
* fix returning of 409 (CONFLICT) when inserting/patching an application with a conflicting `uid`

## Version 0.78.0
* add `order` query parameter for sorting endpoints
* fix default sort order of endpoints to `desc` to match prod

## Version 0.77.0
* fix event_type array query parsing
* fix bad `?channel=` queries
* fire operational webhook on endpoint secret rotation
* implement bidirectional pagination for endpoints

## Version 0.76.1
* fix `/attempt/endpoint`'s broken `?channel=` query

## Version 0.76.0
* add org_id and app_id to main tracing span
* make `wait_for` timeout early and retry
* add since/until query params to endpoint stats
* add endpoints to expunge payload and response
* clarify error message and documentation around filtered IP addresses
* fix error message with endpoint filter types validation
* fix `?after=<time>` ID Parsing in paginated endpoints

## Version 0.75.0
* add built in SSRF support (no need for an external proxy)
* many worker improvements (see #704 for details)
* fix Span HTTP Routes
* update deps
* add Length limit validation to EndpointHeader value
* add missing root CAs to default docker image
* add updated app-portal-access API endpoint (to replace the deprecated dashboard-access)
* add feature flag gating to event types

## Version 0.73.0
* fix issue with potentially incorrect signatures for very small payloads.
* fix Docker image to use `exec` so that svix-server will accept container process signals.

## Version 0.72.0
* add metadata field to Endpoint endpoints
* add missing msg id index causing some queries to be slow
* make sensitive header detection in the `headers` API case insensitive
* omit prevIterator from responses when `null`
* fix string collation for some fields in the db (improves performance)
* expose attempted URL on MessageAttemptOut
* fix graceful server shutdown even when connection to queue is lost
* fix listing and getting event-types permission regression (give app tokens access)
* various code cleanups
* update axum to 0.6

## Version 0.71.0
* Add metadata to application CRUD
* Update Rust, sea_orm, and other deps

## Version 0.70.0
* create subcommand for wiping an organization's data
* internal code improvements

## Version 0.69.0
* ensure QueueTasks are deleted after acknowledgement
* better handling of failures in streaming responses
* update event-type schemas validation to be stricter

## Version 0.68.1
* update the event-type CRUD endpoints to be more strict about the schema type.
* fix typo in error messages for unimplemented errors.

## Version 0.68.0
* add file/line information to errors for easier debugging.
* update dependencies.

## Version 0.67.0
* include idempotency key in log spans.
* correct trace ID handling in tracing spans.

## Version 0.66.0
* add unique IDs when tracing worker tasks.
* allow enabling extra tracing for db operations.
* ensure tracing spans are recorded for all log levels.
* reduce idempotency key lock time from 20s to 5s.

## Version 0.65.1
* fix validation errors to be more informative.
* more strict message payload validation to match the libs.
* fix issue with messages being signed with expired keys in some situations (not a security concern, just superfluous data being sent).
* attach a unique ID to a request (used in logs) when none is provided in headers.

## Version 0.65.0
* support "upsert" of entities on PUT methods.
* support PATCH methods on entities for partial updates.
* allow overriding redis_dsn for separate queue and cache DSNs.

## Version 0.64.2
* change the dashboard authentication payload to note that the server is self hosted.

## Version 0.64.0
* disable endpoints on repeated failures.
* add retry functionality to the Redis queue (to be more resistant minor networking hiccups).
* improve the delayed queue processing to be resilient to errors and better support concurrent processing.
* gracefully shutdown the server on SIGTERM.

## Version 0.63.1

## Version 0.63.0
* add support for encrypting webhook secrets in the database.
* include the error message in the attempt's response for non HTTP errors.
* change the CORS headers to be more strictly compliant.
* wait-for db before attempting to run migrations.
* add retry functionality to Redis cache.

## Version 0.62.0
* add support for asymmetric signatures.
* ensure msg content exists before attempting resend.
* Improve HTTP error response logging.
* fix docker builds due to changes to Redis SSL.

## Version 0.61.0
* add OpenTelemetry support
* send operational ("incoming") webhooks about events happening on the server.
* enable TLS support for Redis
* use correct timestamp for attempt's id and created.
* add jitter to message retry times.
* fix endpoint spelling in some error messages and comments.
* add built in support for waiting for dependencies (database, redis) to be ready (instead of using a script).

## Version 0.60.0
* normalize health status output to be in lowercase.
* implement application portal endpoint and limited access tokens for it.
* add endpoint stats endpoint for getting endpoint statistics.
* add support for JSON log format for use in cloud environments.
* add configuration options for the max pool size for DB and Redis connection pools.
* add CORS headers and support pre-flight requests for use from the browser.
* add a configuration option to only allow https endpoint URLs.
* support passing an org_id when generating JWTs (`svix-server jwt generate`).

## Version 0.59.1
* Libs/Csharp: expose missing getters which prevented a lot of the lib's functionality to be used.

## Version 0.59.0
* make the `health` endpoint more useful by adding more status checks to it.
* parse `retry_schedule` as an array rather than string.
  * People should use the next syntax: `[1, 2]`, though the old syntax is still supported for backwards compatibility.
* add validation to ensure endpoint URL scheme is either http or https.

## Version 0.58.1
* Fix logging configuration in production builds to actually be respected.
* improve how we run Redis migrations.

## Version 0.58.0
* remove `updated_at` field from static entities.
* change to a more efficient first message dispatch.
* change the redis queue to use redis streams.
* serve nice API docs under `/docs`.
* start version tagging the Docker images. (`latest`, `x`, and `x.y`).
* allow PATCH to delete header values by passing null.
* make it possible to change the hard pagination limits to be soft.
* make the redis queue implementation automatically recover from redis crashes.
* add a `--run-migrations` flag to automatically run migrations (flag still passed in Docker).
* fix worker to not follow HTTP redirects when calling webhooks.

## Version 0.57.2
* add support for get-or-create when creating applications.

## Version 0.57.1
* update docker image to use Rust 1.60
* preserve header name capitalization for custom endpoint headers.
* gracefully handle non-textual webhook responses (so endpoints returning non strings).
* delete message content after the specified retention period.

## Version 0.57.0
* add support for idempotency.
* add prev_iterator support to endpoints that should support it.
* make parameter validation stricter in various places.
* improve error messages for bad configurations.
* support for Redis clusters.
* add a memory cache backend.
* fix health endpoint to return an empty response to match its status code (204).
* fix issue that can cause messages to be sent more than once when clients timeout.
* fix support for endpoint id in the list attempted messages and list attempted destinations APIs.

## Version 0.56.0
* extend prev_iterator implementation to allow for before and after.
* update Axum dep to latest version.
* add more tests to the suite.

## Version 0.55.0
* fix marking of some HTTP errors as failed.
* change base docker image to debian-slim
* update deps.
* added missing list attempts endpoints and missing query parameters to some endpoints.
* switch to KsuidMs for extra precision.
* add missing validation for `IdOrUid` so that they return 422.

## Version 0.54.2
* fix release CI round 2

## Version 0.54.1
* fix release CI

## Version 0.54.0
* add more release targets (macOS and aarch64).
* make 422 errors compatible with the Svix service.

## Version 0.52.0
* Remove deprecated axum API usage
* Implement PATCH for endpoint headers API
* Add support for an endpoint cache for improved performance when sending

## Version 0.50.0
* Set custom endpoint headers when sending via worker

## Version 0.49.0
* Add support for adding & managing custom Endpoint Headers

## Version 0.47.0
* **Initial OSS release of Svix Server!** 🚀
