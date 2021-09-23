# Changelog

## Version 0.30.0
* Kotlin: Initial async kotlin library 🚀
* All: Improved naming for Integer Enums 📛
* JS: Update dependencies 🆕
* Java: Cleanup list option code using generics 🤓

## Version 0.29.0
* All: Add before option to message apis
* Java: Cleanup list options
* All: Add ability to set signing secret
* All: add event type filtering to message attempt api
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
* Go: Expose new error class for checking api errors
* PHP: Fix issue with psr-4 autoloading
* Python: Properly export WebhookVerificationError
* Python: Fix bug in event_type create function

## Version 0.17.0
* Migrate to new github org `svix`! :rocket:

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
