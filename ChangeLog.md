# Changelog

## Note: Server and Bridge changelogs

The Svix Server changelog has moved to [server/ChangeLog.md](./server/ChangeLog.md).
The Svix Bridge changelog has moved to [bridge/ChangeLog.md](./bridge/ChangeLog.md).

## Unreleased

## Version 1.99.1
* Libs/JavaScript: Fix empty `retryScheduleInMs` not being honored
* Libs/JavaScript: Fix `0` entries in `retryScheduleInMs` not being honored

## Version 1.99.0
* CLI: Use [wolfi](https://github.com/wolfi-dev) as the image base instead of Debian, shrinking the Docker image by 77% and removing all open CVEs
* Libs/C#: Fix a bug that made the 'canceled' message-attempt status opt-in unreliable (see also changelog for 1.95.0)

## Version 1.98.0
* CLI: Set OCI metadata on Docker images
* CLI: Embed package metadata in compiled binaries on Linux

## Version 1.97.0
* CLI: Ignore `EPIPE` when printing output
* Libs/All: Support customizing expiration (grace period) of old endpoint secret when rotating
* Libs/Python: Bump minimum-supported Python interpreter version to 3.9
* Libs/Python: Fix memory and file-descriptor leak from excessively constructing httpx clients

## Version 1.96.1
* Libs/Java: Upgrade jackson dependency to v2.21.4
* Libs/Rust: Make `MessagePollerv2Consumer*Options` public

## Version 1.96.0
* CLI **(New)**: Add `--token` option to `svix listen`
* Libs **(New)**: AutoConfigConsumer SDK to consume Polling Endpoints
* Libs/Csharp: Accept a valid signature when it is last in a multi-sig header (thanks [@devteamaegis])
* Libs/All: Stop sending with_content parameter to the backend, handle it client-side

## Version 1.95.2
* Libs/Kotlin: Webhooks AutoConfig SDK
* Libs: Add `health.get` to Go, Python and Rust SDK
  * It was previously missing from these SDKs for historical reasons
* Libs/All: Add versions in user-agent header
* Libs/Go: Add additional internal management APIs

## Version 1.95.1
* Libs/Python: Fix release workflow (1.95.0 of the Python SDK failed publishing)

## Version 1.95.0
* Libs/All: Enable server-side support of 'canceled' message-attempt status by default
  * If you were previously comparing attempt statuses against 'success', note that the
    result of the comparison will now change for messages canceled by a transformation script
* Libs/Python: Move from `setup.py` to [PEP-518](https://peps.python.org/pep-0518/)-compliant `pyproject.toml` build system
* Libs/Python: Clarify that the minimum-supported Python interpreter version is 3.8
* Libs/Python: Actually run tests against all supported python versions
* Libs/PHP: Fix deprecation warning for `DateTimeImmutable` construction from `null` (thanks [@VincentLanglet])

[@VincentLanglet]: https://github.com/VincentLanglet

## Version 1.94.0
* Libs **(New)**: Webhooks AutoConfig SDK
* Libs/Java: Restore compatibility with Java 1.8 in Webhooks verification

## Version 1.93.0
* CLI: Show input / output JSON examples in `--help` output
* Libs/Java: Restore compatibility with Java 1.8

## Version 1.92.2
* Libs/JavaScript: Remove dependency on `uuid` (thanks [@ulrichstark])

[@ulrichstark]: https://github.com/ulrichstark

## Version 1.91.1
* Fix docker build

## Version 1.91.0
* Libs/Ruby: various fixes to the ruby sdk

## Version 1.90.0
* Libs/All: Update to latest Svix Cloud spec

## Version 1.89.0
* Libs/Python: Set `__str__` on errors

## Version 1.88.0
* Libs/Go: Add support for `v1.management.authentication.patch-api-token` (Internal endpoint)
* Libs/Go: Add TransportWrapper option to SvixOptions (thanks [@piotrdomagalski])

[@piotrdomagalski]: https://github.com/piotrdomagalski

## Version 1.86.0
* Libs/All: Add support for `v1.message.create-precheck` (Svix Cloud exclusive endpoint)

## Version 1.85.0
* Libs/Ruby: Return nil if payload is empty, instead of attempting to parse as JSON (thanks [@donovanlopez])
* Libs/Rust: Bump MSRV to 1.88.0
* CLI: Bump MSRV to 1.88.0

[@donovanlopez]: https://github.com/donovanlopez

## Version 1.84.1
* Libs/JavaScript: Updated webhook verification logic to use official standard webhooks packages

## Version 1.84.0
* Libs/Python: Updated webhook verification logic to use official standard webhooks packages
* CLI: support disabling TLS verification when relaying requests (`svix listen`)

## Version 1.83.0
* Libs/PHP: Add `MessageIn::createRaw`, allows you to create a raw (non-json) message

## Version 1.82.0
* Libs/All: Add support for connector UIDs
* CLI: Add support for the Connector API
* CLI: Add `-v --verbose` flag
* CLI: Fix a bug in config file saving

## Version 1.81.0
* Libs/All: Add support for the new [Connector API](https://api.svix.com/docs#tag/Connector)
  (see also [the corresponding docs section](https://docs.svix.com/connectors))
* CLI: Fix `svix listen` failing with "Could not automatically determine the process-level CryptoProvider"

## Version 1.80.0
* Libs/PHP added support for `$svix->application->getOrCreate`

## Version 1.79.0
* Libs/Kotlin: Add `SvixOptions` to the `com.svix.kotlin` namespace
  * The un-namespaced symbol is kept as a `typealias` for backwards compatibility
* Libs/Javascript: Add option to configure the fetch method

## Version 1.78.0
* Libs/Python: Fix bug preventing the generic webhook ingest source from being deserialized
* Libs/Python: Add API for the new [Svix Stream](https://www.svix.com/stream/)

## Version 1.77.0
* Libs/All: Add API for the new [Svix Stream](https://www.svix.com/stream/)
* Libs/PHP: Fix bug causing empty objects to be serialized as `[]` instead of `{}`
* Libs/Rust: Upgrade rustls dependency version (thanks @GodTamIt)

## Version 1.76.1
* Libs/Go: Fix bug causing 422 errors on `message.create` with messages created using the `NewMessageInRaw` helper

## Version 1.76.0
* Libs/PHP: Added support for the full Svix SDK!

## Version 1.75.1
* Libs/Go: Fix bug preventing `time.time` query params from serializing correctly
* Libs/All: Allow deleting endpoint headers through new `deleteHeaders` field in `EndpointHeadersPatchIn`

## Version 1.75.0
* CLI: Modify Dockerfiles to use cache mounts for improved build time; these now require Docker 1.2 or later to build
* Libs/JS: Add support for custom retry schedule (thanks @KranzAklilu)

## Version 1.74.1
* CLI: Fix installation of ca-certificates in Docker images
  * These images were broken as of v1.72.0

## Version 1.74.0
* Libs/Rust: Add support for custom retry schedule (thanks @KranzAklilu)
* Libs/Rust: Add support for connecting to the API through a proxy

## Version 1.73.0
* Libs/(Java and Kotlin): Fix bug causing runtime exceptions when unknown fields were sent from the server

## Version 1.72.0
* Libs/JavaScript: Use native `fetch` API

## Version 1.70.1
* Libs/All: Re-add `endpoint.transformationPartialUpdate` as a deprecated operation
  * It was removed through a rename in the previous version, which should not have happened

## Version 1.70.0
* CLI: Add `svix seed` command to create testing resources (thanks @KranzAklilu)
* Libs/Go: Fix request retries not working (thanks @mixnblend!)
* Libs/All: Revert accidental addition of some authentication APIs
* Libs/All: Fix name and input structure for rarely used set-transformation route
  * Rename `endpoint.transformationPartialUpdate` to `endpoint.patchTransformation`
  * Replace `EndpointTransformationIn` with `EndpointTransformationPatch`
    * In some but not all the SDKs, these have slightly different field types
  * This is a breaking change, but based on server statistics this method is barely used by anybody

## Version 1.69.0
* Libs/Rust: Don't panic on invalid inputs to request building

## Version 1.68.0
* Libs/PHP: Handle Badly Formatted Signatures by @rodnaph in https://github.com/svix/svix-webhooks/pull/1942
* Libs/All: Automatically send an idempotency key on all outgoing post requests

## Version 1.67.0
* Libs/Go: Disable HTTP/2 for API calls because it was causing issues for some users
* Libs/Python: Enable configuring a proxy for API requests
* Libs/All: Add support for PandaDoc ingest source

## Version 1.66.0
* Libs/JavaScript: Fix response processing code for endpoints with an optional datetime field in the response body
* Libs/PHP: Update minimum version of PHP to 8.0

## Version 1.65.0
* Libs/Python: Bring back the (deprecated) sync `dashboard_access` method, which was accidentally
  removed in v1.64.1
* Libs/Csharp: The `options` argument to the `SvixClient` initializer is now optional.
* Libs/Csharp: The `SvixOptions.BaseUrl` field is deprecated in favor of `SvixOptions.ServerUrl`
* Libs/(Ruby and Kotlin): Add doc comments to class attributes
* Libs/Go: Added a new `<Enum>From<UnderlyingType>` map to all enums. For example `BackgroundTaskStatusFromString["running"]` will result in `BACKGROUNDTASKSTATUS_RUNNING`
* Libs/Go: Fixed bug where the correct `content-type` was not set on `PUT` requests

## Version 1.64.1
* Libs/JavaScript: Add `HTTPValidationError`, `HttpErrorOut`, `ValidationError` and `ApiException` to the top level exports.
* Libs/Python: Specify minimum version of pydantic `pydantic >=2.10` in setup.py.

## Version 1.64.0
* CLI: Add interactive login with dashboard.svix.com

## Version 1.63.2
* Libs/JavaScript: Fix deserialization for optional arrays
* Libs/Python: Fix minimum version requirement for pydantic

## Version 1.63.1
* Libs/Python: Removed internal use of `match` statement, allowing python versions older than 3.10 to be used

## Version 1.63.0
* Libs, Cli: New Ingest source and Ingest endpoint APIs are now available.
* Libs: New environment APIs (import, export) are now available.
* Libs/Ruby: Fix bug causing `application.get_or_create` to raise a `TypeError`

## Version 1.62.0
* Libs/Python: The client will now reuse the same httpx Client across api calls

## Version 1.61.4
* Libs/(C#,Java,Kotlin,Rust): The type of `BackgroundTaskFinishedEvent2.data` changed from `Data` (a custom type) to `Object`(Java C#), `serde_json::Value`(Rust), `Map<String, Any>`(Kotlin)

## Version 1.61.3
* Libs/Javascript: Fix bug where a missing content-type header would cause some requests to fail

## Version 1.61.2
* Libs/Java and Libs/Kotlin: Fix bug introduced in v1.61.1, where `message.create` would return 422 on all requests

## Version 1.61.1
* Libs/Java **(Breaking)**: The type of `MessageIn.transformationsParams` changed from `Object` to  `Map<String,Object>`.
* Libs/Java and Libs/Kotlin **(Breaking)**: Due to an internal change in the underlining JSON de/serialization library, some JSON objects may not be serialized correctly. To address this `MessageIn.payload` now accepts a JSON encoded string instead of an `Object` (for Java) or `Map<String, Any>` (for Kotlin)
* Libs/Go: Fix regression in Go lib, operations now return `&svix.Error` instead of `svix.Error`

## Version 1.61.0
* Libs/Ruby **(Breaking)**: Ruby version changed from `2.7` to `3.4.2`
* Libs/Ruby **(Breaking)**: Deprecated methods `MessageAttempt.list` and `MessageAttempt.list_attempts_for_endpoint` are removed

## Version 1.60.1
* Libs/Javascript: Fix regression causing json deserialization to throw exception in a few edge cases

## Version 1.60.0
* Libs/Java **(Breaking)**: All uses of `ListOptions`/`PostOptions` are removed, and renamed to `{Resource}{Operation}Options`. For example in `Endpoint.List` you would now use `EndpointListOptions`
* Libs/Java **(Breaking)**: Deprecated methods `MessageAttempt.list` and `MessageAttempt.listAttemptsForEndpoint` are removed

## Version 1.59.2
* Libs/Go: Fixed regression where go >= 1.23 was needed, now Go >= 1.21 is supported

## Version 1.59.1
* Libs/JavaScript: Fixed regression introduced in version 1.59.0, types are now correctly exported

## Version 1.59.0
* Libs/Kotlin **(VERY IMPORTANT)**: The parameter order `appId` and `msgId` were swapped on `Message.get` and `Message.expungeContent`
* Libs/Kotlin **(Breaking)**: All uses of `ListOptions`/`PostOptions` are removed, and renamed to `{Resource}{Operation}Options`. For example in `Endpoint.List` you would now use `EndpointListOptions`
* Libs/Kotlin **(Breaking)**: In the 4 `*Patch` patch models, nullable fields are of type `MaybeUnset<T>` instead of `T`. call `MaybeUnset.Present(val)` to initialize this value
* Libs/Kotlin **(Breaking)**: `SvixOptions` no longer has `initialRetryDelayMillis` or `numRetries` instead use `retrySchedule`
* Libs/Kotlin **(Breaking)**: All `{Resource}{Operation}Options` and model classes (`ApplicationIn`/`MessageOut`) are now data classes
* Libs/Kotlin **(Breaking)**: Deprecated functions `MessageAttempt.list` and `MessageAttempt.listAttemptsForEndpoint` are removed
* Libs/Kotlin **(Breaking)**: All uses of `java.time.OffsetDateTime` replaced with `kotlinx.datetime.Instant`
* Libs/Kotlin **(Breaking)**: All uses of `java.net.URL` in request/response models are replaced with `String`
* Libs/JavaScript **(Breaking)**: Removed non-public `endpoint.oauthUpdate` and `endpoint.oauthDelete` operations.
* Libs/JavaScript **(Breaking)**: Removed deprecated operation `MessageAttempt.list`
* Libs/JavaScript **(Breaking)**: Exceptions and models are no longer exported from `webhook.ts`, import them from `index.ts` instead

## Version 1.58.2
* Libs/Go: New `Message.ExpungeContent` is now available
* Libs/Go: Fix regression introduced in version `1.58.0`. All models are now exported from the `svix` package.
* Libs/Go: Fix regression introduced in version `1.58.0`. Nullable fields are now `*T` instead of `T`, this affects `ListResponseXXXOut.Iterator`

## Version 1.58.1
* Libs/Go: Fix a null-pointer deference regression introduced in previous version.

## Version 1.58.0
* Libs/Rust: Fix regression in previous version. Nullable fields are now `Option<T>` instead of `T`, this affects `ListResponseXXXOut.iterator` and `EnvironmentOut.settings`

## Version 1.57.0
This version contains a big overhaul of the client libraries, with improved typing.

* Libs/Go: Add `Authentication.ExpireAll` (and `ExpireAllWithOptions`)
* Libs/Go **(Breaking)**: Excluding specific fields on the *Patch models (`ApplicationPatch` for example), all `Nullable{Type}` removed from the models
* Libs/Go **(Breaking)**: All `Nullable{Type}` (for example `NullableString`) are replaced with a new generic `Nullable[T]` type, the new type can be imported from `github.com/svix/svix-webhooks/go/utils`
* Libs/Go **(Breaking)**: All custom model types are now imported from `github.com/svix/svix-webhooks/go/models` instead of `github.com/svix/svix-webhooks/go`
* Libs/Go **(Breaking)**: All `-WithOptions` methods are now removed. Their regular counterparts now take a pointer to an Options type which can be nil when not needed. For example in `Endpoint.RecoverWithOptions` is now `Endpoint.Recover`

* Libs/C# and Libs/Go **(Breaking)**: All uses of `ListOptions`/`PostOptions` are removed, and renamed to `{Resource}{Operation}Options`. For example in `Endpoint.List` you would now use `EndpointListOptions`
* Libs/C# **(Breaking)**: All `IdempotencyKey` method parameters are removed, and are now part of `{Resource}{Operation}Options`. For example in `Message.Create`; to the use `IdempotencyKey`, simply pass it in the `MessageCreateOptions`
* Libs/C# **(Breaking)**: The `Throw` parameter is removed from `SvixOptions`
* Libs/C# **(Breaking)**: All redundant interfaces along with the `Svix.Abstractions` namespace are removed
* Libs/C# **(Breaking)**: The `Svix.Model` and `Svix.Models` namespaces are now merged into a single `Svix.Models` namespace
* Libs/C# **(Breaking)**: The `Svix.Client` namespace is removed, The `SvixClient` class can now be found in the `Svix` namespace

* Libs/Python **(Breaking)**: `PostOptions` and `ListOptions` are no longer used in methods for `Authentication`,`Endpoint`,`EventType`,`Integration`,`MessageAttempt`,`Message`, `Statistics` and `OperationalWebhookEndpoint` resources. Instead each API call now has its own `{Resource}{Operation}Options`. (Both sync and async)
* Libs/Python: In `Application` the `dashboard_access` method is deprecated in favor of `app_portal_access`. (Both sync and async)
* Libs/Python **(Breaking)**: `EndpointStatsOptions` is renamed to `EndpointGetStatsOptions`
* Libs/Python **(Breaking)**: `MessageAttemptListOptions` is removed in favor of call specific `{Resource}{Operation}Options`
* Libs/Python **(Breaking)**: For `Statistics` in the `aggregate_event_types` method the `task_id` parameter is removed, Please note that previously this parameter was ignored and had no affect (Both sync and async)

* Libs/Kotlin **(Breaking)**: Update `recover` to return `RecoverOut` (instead of nothing)
* Libs/Kotlin **(Breaking)**: Update `replayMissing` to return `ReplayOut` (instead of nothing)
* Libs/Kotlin **(Breaking)**: Update `sendExample` to return `MessageOut` (instead of nothing)
* Libs/Kotlin **(Breaking)**: Update `MessageAttempt` list methods to each have its own type for
  list options, since they don't all support the exact same set of parameters and some of the
  parameters that could be set before would just get ignored
* Libs/Kotlin: Fix a bug in `EventType.list` where `options.order` was not getting honored

* Libs/Rust **(Breaking)**: Add optional `EventTypeDeleteOptions` parameter to `EventType::delete`
* Libs/Rust **(Breaking)**: Add optional `options` parameters to `Endpoint::recover`,
  `Endpoint::rotate_secret`, `Integration::rotate_key` and `MessageAttempt::resend`
* Libs/Rust **(Breaking)**: Remove model files that were not referenced by any operations available
  through the `Svix` object
* Libs/Rust **(Breaking)**: Switch `Patch` struct fields from `Option<Option<T>>` to
  `js_option::JsOption<T>`
* Libs/Rust **(Breaking)**: Change `rate_limit` from `i32` to `u16` in several places
* Libs/Rust **(Breaking)**: Remove `settings` parameter from `EnvironmentIn::new`
* Libs/Rust **(Breaking)**: Replace `PostOptions` with operation-specific options structs
* Libs/Rust **(Breaking)**: Remove `Period` from `BackgroundTaskType` variant names; this was introduced by accident a few releases ago

## Version 1.56.0
* Skipping versions: we had an issue with our CI that created duplicated Go
  library releases and forced us to bump the version across the libs and the
  server. Apologies for any inconvenience caused.
* Libs/Go: retracts several versions which were tagged prematurely. Apologies!

## Version 1.45.1
* CLI: Rename the binary back from `svix-cli` to `svix`
* Libs/Java: Stop sharing one `ApiClient` between all `Svix` instances. Fixes a bug where multiple
  independently instantiated `Svix` instances would always use the same base path and auth token.

## Version 1.45.0
* CLI **(New)**: New Rust-based CLI with support for all recent Svix functionality.
* Libs/Rust **(Breaking)**: Add `api::MessageAttemptListAttemptedMessagesOptions` and use it for
  `MessageAttempt::list_attempted_messages`, replacing `MessageAttemptListOptions` which contained
  some extra parameters never used with this method / endpoint ([#1568])
* Libs/JavaScript **(Breaking)**: Add more precise type annotations for `options` parameters on
  `MessageAttempt.list`, `MessageAttempt.listByMsg`, `MessageAttempt.listAttemptedMessages` and
  `MessageAttempt.listAttemptedDestinations` ([#1571])
* Libs/JavaScript **(Breaking)**: Rename `EndpointStatsOptions` interface to
  `EndpointGetStatsOptions` ([#1585])
* Libs/Kotlin **(Breaking)**: Remove `ListOptions` class. Usage of classes that were inheriting
  from it should not change though ([#1587])
* Libs/Rust: Add `api::Authentication::expire_all` ([#1584])
* Libs/Rust: Rename some `Options` types. The old names remain as deprecated type aliases ([#1584])

[#1568]: https://github.com/svix/svix-webhooks/pull/1568
[#1571]: https://github.com/svix/svix-webhooks/pull/1571
[#1584]: https://github.com/svix/svix-webhooks/pull/1584
[#1585]: https://github.com/svix/svix-webhooks/pull/1585
[#1587]: https://github.com/svix/svix-webhooks/pull/1587

## Version 1.44.0
* Libs/JavaScript: Revert packaging-related change because it broke for some users ([#1556])
* Libs/Rust **(Breaking)**: Remove unused optional `PostOptions` parameters from non-POST methods ([#1557])

[#1556]: https://github.com/svix/svix-webhooks/pull/1556
[#1557]: https://github.com/svix/svix-webhooks/pull/1557

## Version 1.43.0
* Libs/Go: Add convenient construction of messages with pre-serialized payload ([#1538])
* Libs/Go: Reduce the use of `NullableX` types to where they actually have a use ([#1543])
* Libs/JavaScript: Add convenient construction of messages with pre-serialized payload ([#1539])
* Libs/Java: Add convenient construction of messages with pre-serialized payload ([#1544])
* Libs/C#: Bump .NET target to 8.0 ([#1546])
* Libs/C#: Add convenient construction of messages with pre-serialized payload ([#1545])
* Libs/Python: Add convenient construction of messages with pre-serialized payload ([#1540])
* Libs/Ruby: Add convenient construction of messages with pre-serialized payload ([#1541])
* Libs/JavaScript: Compile svix package for ESM and CommonJS, reducing bundle sizes ([#1549])

[#1538]: https://github.com/svix/svix-webhooks/pull/1538
[#1543]: https://github.com/svix/svix-webhooks/pull/1543
[#1539]: https://github.com/svix/svix-webhooks/pull/1539
[#1540]: https://github.com/svix/svix-webhooks/pull/1540
[#1541]: https://github.com/svix/svix-webhooks/pull/1541
[#1544]: https://github.com/svix/svix-webhooks/pull/1544
[#1545]: https://github.com/svix/svix-webhooks/pull/1545
[#1546]: https://github.com/svix/svix-webhooks/pull/1546
[#1549]: https://github.com/svix/svix-webhooks/pull/1549

## Version 1.42.0
* Libs/Python: Fix sync / async mismatch for op-webhook-endpoint API ([#1535])
* Libs/Rust: Fix types of `iterator` fields ([#1534])
* Libs/Kotlin: Add convenient construction of messages with pre-serialized payload ([#1531])
* Libs/Rust: Add convenient construction of messages with pre-serialized payload ([#1533])

[#1531]: https://github.com/svix/svix-webhooks/pull/1531
[#1533]: https://github.com/svix/svix-webhooks/pull/1533
[#1534]: https://github.com/svix/svix-webhooks/pull/1534
[#1535]: https://github.com/svix/svix-webhooks/pull/1535

## Version 1.41.0
* Libs/JS: Allow `iterator` and date parameters on list endpoints to be `null` (in addition to `undefined`)
* Libs/JS: Fix serialization of message `status` query parameters
* Libs/Rust: Revert many pointless type changes from `Option<T>` to `Option<Option<T>>` that came with 1.39.0

## Version 1.40.0
* Libs(JS): downgrade `@stablelib/base64` avoiding `ERR_REQUIRE_ESM` by @svix-onelson in https://github.com/svix/svix-webhooks/pull/1506

## Version 1.39.0
* Libs: Add operational webhook endpoint API
* Libs/Rust: **[Breaking]** Models for PATCH endpoints now have `Option<Option<T>>` fields to allow explicitly sending nulls to unset those fields.
* Libs/Go: **[Breaking]** Nullable arrays are once again represented by `[]T` instead of `*[]T`. Serialization handles the empty case correctly.
* Libs: Upgrade `openapi-generator` to 7.9.0, with dependency upgrades and internal changes in the SDKs.

## Version 1.37.0
* Libs(Go): `EndpointUpdate` and `EndpointPatch` allow null for channels, filter types

## Version 1.36.0
* Libs(Go): edit `EndpointIn` to allow null for channels, filter types

## Version 1.35.0
* Libs/Python: Revert breaking change to MessageInPayload/MessageOutPayload

## Version 1.34.0
* Libs/Rust: Add missing export of `V1MessageEventsParams`

## Version 1.32.0
* Libs/Go Remove modules no longer produced by the generator

## Version 1.31.0
* Libs/Go: prune out unused import for "time" in codegen output
* Libs/C#: Fix appId, endpointId order in RotateSecretWithHttpInfoAsync
* Libs: Support passing `application` to `app-portal-access` endpoint

## Version 1.29.0
* Libs/C#: add GetAppPortalAccess* methods to IAuthentication interface
* Libs/Go: expose `with_content` for `Message.List`
* Libs/Go: expose `with_msg` param on `MessageAttempt.ListByEndpoint`
* Libs/Go: expose `order` in `Integtation.List` and `EventType.List`
* Libs/Go: return response body for recover/replay
* Libs/Go: expose `tag` for `MessageAttempt` list endpoints
* Libs/Javascript: Publish src to npm to make it available for source maps
* Libs/Python: ensure datetimes have a timezone

## Version 1.28.0
* Libs: Update OpenAPI spec

## Version 1.27.0
* Libs/Python: add missing 'get_stats' function

## Version 1.25.0
* Libs/Javascript: Apply workaround for incomplete fetch support in Cloudflare Worker
* Libs/Go: Add NullableBool function
* Libs/Python: Update dependencies and switch to ruff
* Libs/Rust: Add `Svix::with_token` to allow changing API token
* Libs/PHP: Replace ctype_digit for PHP 8.1 deprecation of non-string arguments

## Version 1.24.0
* Libs: Update OpenAPI

## Version 1.23.0
* Libs/Javascript: Fix method naming consistency
* Add oauth support to javascript

## Version 1.22.0
* Libs/Ruby: Add PATCH endpoints

## Version 1.21.0
* Libs/Rust: Print more detailed error messages for generic errors

## Version 1.20.0
* Libs/C#: don't serialize nulls in PATCH endpoints
* Libs/Rust: Make API method Future implement Send/Sync

## Version 1.19.0
* Libs/Python: Make request timeout and retries configurable
* Libs/Rust: Make request timeout configurable

## Version 1.18.0
* Libs/C#: **[Breaking]** Return iterator information in list endpoints. Changes the return type of list endpoints.
* Libs/Java: don't serialize nulls in PATCH endpoint methods
* Libs/Rust: upgrade and clean up dependencies
* Libs/Rust: switch from reqwest to hyper 1.0

## Version 1.17.0
* Libs/Rust: **[Important]** Fix a bug in the webhook signature verification method where certain signatures could bypass the verification.
* Libs/Java: **[Breaking]** Use Java time instead of threetenbp. This removes the need to import threetenbp to use the library. Depending on how the lib is used, it might require migrating uses of threetenbp to Java 8 Date-Time APIs.

## Version 1.16.0
* Libs: update OpenAPI spec and libs.
* Libs/Javascript: Fix thrown error message when API status code is unknown.

## Version 1.15.0
* Libs: update OpenAPI spec and libs.

## Version 1.14.0
* Libs: expose two new Statistics endpoints.
* Libs/Csharp: implement "hard delete" event types.
* Libs/Ruby: accept additional options on message get.
* Libs/Go: fix transformation partial update function.
* Libs/Kotlin: fix errors with APIs that return no content (like endpoint delete).

## Version 1.13.0
* Libs/Go: expose `EventType.DeleteWithOptions`

## Version 1.12.0
* Libs: update OpenAPI spec

## Version 1.11.0
* Libs: add functions to import event types from OpenAPI specs
* Libs: update OpenAPI spec

## Version 1.9.0
* Libs/Go: export missing types

## Version 1.8.1
* Libs: update OpenAPI spec

## Version 1.8.0
* Libs/Go: fix behavior of `NullableInt32` and `NullableString`

## Version 1.7.0
* Libs/Rust: fix handling of integer enums (was causing failures for some endpoints).

## Version 1.6.0
* Libs: implement the `patch` endpoints for partial updates

## Version 1.5.2
* Libs/JavaScript: Fix regression in previous release and allow passing `Buffer` to sign/verify.

## Version 1.5.1
* Libs/JavaScript: Add an explicit check that payload is a string.
* Libs: Fix a bug with integration key creation.

## Version 1.5.0
* Libs/Go: support passing `WithContent` to `List Attempted Messages`
* Libs/Python: fix regression in Python library (causing some functions not to work).

## Version 1.4.12
* Sync OSS package version with our internal version.
* Libs: update OpenAPI spec
* Libs: add support for "Background Task" endpoints
* Libs: add support for since/until to endpoint stats
* Libs/Go: add missing `endpoint_id` option to list attempts by msg.
* Libs/JavaScript: add missing `withContent` field to `MessageListOptions`.
* Libs/Rust: fix typo in error handling message

## Version 0.85.1
* Libs: update OpenAPI spec

## Version 0.85.0
* Libs/Ruby: fix the library failing to load due to missing dependency.
* Libs: **[Semi-breaking]** we changed the return value of the transformation simulation endpoint. It's technically a breaking page, though the API is private so it shouldn't affect people.

## Version 0.84.1
* Libs: regenerate libs from the correct openapi spec (could have caused potential authentication issues)

## Version 0.84.0
* Libs/Rust: support choosing the wanted TLS implementation

## Version 0.83.1
* Libs: update OpenAPI spec

## Version 0.83.0
* Libs: Add `send-example` wrapper to libraries
* Libs/Go: alias missing types from internal dir / openapi package

## Version 0.82.1
* Libs/Java: serialize nulls when making HTTP requests

## Version 0.81.0
* Libs: add support for creating application when creating a message
* Libs/Go: bump Go version

## Version 0.80.0
* Libs: add `prev_iterator` and `order` support to application list
* Libs: add `prev_iterator` support to event type list
* Libs/C#: **[Breaking]** change default value for `SvixOptions.Throw` to `true`

## Version 0.79.0
* Libs/Ruby: require (reexport) app portal models in ruby (fixing errors)
* Libs/C#: fix MessageAttempt querying when not filtering by status and code

## Version 0.78.0
* Libs: add support for `prev_iterator` for application and endpoints
* Libs/JS: fix sign function to support non-round dates
* Libs/Go: **[Breaking]** accept a context parameter in all Go lib methods

## Version 0.77.0
* Libs/Rust: glob-reexport all generated models in Rust

## Version 0.76.1
* Libs/Rust: add missing exports to a few API endpoints
* Libs: fix naming of replay-missing methods in libraries (all but Rust)

## Version 0.76.0
* Libs: add endpoints to expunge payload and response
* Libs: add replay missing messages functions
* Libs: add transformations APIs

## Version 0.75.0
* Libs/Rust: Enable TLS support in `reqwest` by default
* Libs: support the new feature flag gating of event types

## Version 0.74.1
* Libs/Rust: make all `*Options` and fields public (missing from previous release)

## Version 0.74.0
* Libs/Rust: make `ListOptions` and `PostOptions` fields public
* Libs/Rust: fix Webhook verification to accept &str

## Version 0.72.0
* JavaScript: make signature comparison constant time (thanks @arjunyel)

## Version 0.71.0
* Libs: Update OpenAPI spec

## Version 0.70.0
* Libs/Java: fix issue with automatic region detection not working

## Version 0.69.0
* Libs/Kotlin: support configuring retry schedule
* Libs: fix metadata field in Go and Python

## Version 0.68.1
* Libs/Java: fix issue with creating multiple Svix instances in parallel.

## Version 0.68.0
* Libs: add application/endpoint metadata fields.
* Libs/C#: Add netstandard2.0 support.
* Add gitleaks config to ignore test auth tokens.

## Version 0.66.0
* Libs: expose endpoint stats function.

## Version 0.65.1
* Libs/Python: fix typo in Python lib causing auto-detection of EU servers to fail.
* Libs/C#: make logger optional when creating SvixClient.

## Version 0.65.0
* Libs: support "upsert" of entities on PUT methods.
* Libs/Ruby: fix region auto-detection.

## Version 0.64.2
* Libs/JavaScript: fix issue when signing/verifying payloads with high Unicode codepoints (e.g. some new emoji)

## Version 0.64.1
* Libs/Go: export missing `svix.NullableString` utility.

## Version 0.64.0
* Libs/Kotlin: fix compilation issues.
* Libs: automatically detect region (and URL) from auth token.

## Version 0.63.1
* Lib/JavaScript: fix setting string webhook secrets.

## Version 0.63.0
* Libs: update libraries to accept a raw webhook secret.

## Version 0.62.1
* Libs: fix Kotlin and Java build

## Version 0.62.0
* Libs: fix nullable fields to be marked as such (fixes parsing errors in some clients).

## Version 0.60.0
* Lib/Python: update httpx dependency to the latest version.
* Lib/Rust: fix mixup with validation errors marked as HTTP errors and vice-versa.

## Version 0.59.1
* Libs/Csharp: expose missing getters which prevented a lot of the lib's functionality to be used.

## Version 0.59.0
* Libs/Python: show a more useful error for obviously malformed secrets.
* Libs/JavaScript: show a more useful error for obviously malformed secrets.

## Version 0.58.2
* Libs: update OpenAPI spec

## Version 0.58.0
* Lib/Rust: add a Rust API client + webhook verification library!
* Lib/Python: fix package installation on Windows.
* Lib/Csharp: make some parameters optional for better ergonomics.

## Version 0.57.2
* Libs/C#: fix library compilation (broke in the previous release.

## Version 0.57.1
* Libs: fix all libraries to handle 429 (rate limiting).

## Version 0.57.0
* Libs/C#: add C# API client library.

## Version 0.56.0
* Libs/Python: fix user agent to actually work.
* Libs/Python: increase read timeout.

## Version 0.54.0
* Libs: add retry and request-id headers for easier debugging.

## Version 0.53.2
* Ruby: Fix CI to not include Vendor data in package.

## Version 0.53.1
* Java & Kotlin: Build against java 11
* Ruby: Fix a broken import (we changed the name of a webhook event) 🐞

## Version 0.53.0
* Python: **Breaking** The python library is now fully typed!  There may be some breaking changes related to this upgrade (including dropping support for Python 2.X). Please check test and check your integration before upgrading to this version. 🤓
* Python: New Async API via SvixAsync! 🚀
* Libs: Remove the `prevIterator` option from message attempt options (This was added by mistake, never worked and was never meant to work. All iterators should be passed via the `iterator` option. Sorry for the confusion!)
* Libs: Allow filtering by messages by channel

## Version 0.52.0
* Libs/JS: Fix for setting Idempotency-Key when one isn't set  🐞
* Libs/All: Add `msg_id` to attempt list responses (`MessageAttemptOut`)
* Libs/All: New GetOrCreate application function 🆕

## Version 0.51.0
* Libs: Support for passing an idempotency key to post commands
* Libs: Add support for filtering by channel
* Kotlin & Go: Support filtering by event type
* JS: fix for API calls using a default idempotency key when one isn't set 🐞

## Version 0.50.0
* Libs: Add support for filtering by StatusCodeClass in attempts API

## Version 0.49.0
* Libs: Add support for reverse iteration (prev_iterator) & after param in list commands ⬅️

## Version 0.48.0
* Libs: Support for new `/attempt/` API via list_for_msg and list_for_endpoint. 🚀
* **Deprecation warning:** `message_attempt.list` is deprecated in favor of this new API. ❌

## Version 0.47.0
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
