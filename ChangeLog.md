# Changelog

## Next
* 

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
