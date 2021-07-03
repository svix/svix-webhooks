# Changelog

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
