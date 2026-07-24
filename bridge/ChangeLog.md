# Changelog

## Unreleased
* Add an opt-in Kafka record envelope for JSON transformations
* Allow configuring Kafka consumer `auto.offset.reset`

## Version 1.99.0
* Include the Kafka partition in consumer idempotency keys

## Version 1.98.0
* Set OCI metadata on Docker images
* Embed package metadata in compiled binaries on Linux

## Note: missing versions

Before v1.100.0 of svix-bridge, bridge versions were synced with SDK versions and we published
releases even when the bridge component had no changes since the prior release.
That is why version numbers are not contiguous from here on.

## Version 1.89.0
* Add support for PLAIN sasl auth for confluent cloud
* Build Docker image with Rust 1.94

## Version 1.75.0
* Modify Dockerfiles to use cache mounts for improved build time; these now require Docker 1.2 or later to build

## Version 1.74.1
* Fix installation of ca-certificates in Docker images
  * These images were broken as of v1.72.0

## Version 1.74.0
* Add support for connecting to the Svix API through a proxy

## Version 1.72.0
* Add a default `user-agent` to requests made by the new `http` output
* Upgrade Docker base image to Debian Trixie

## Version 1.71.0
* Add `http` output to `receivers`
  * See [`bridge/svix-bridge.example.receivers.yaml`](./bridge/svix-bridge.example.receivers.yaml) for a usage example

## Version 1.65.0
* Add `/health` endpoint by @CodeMan62 in https://github.com/svix/svix-webhooks/pull/1903

## Version 1.40.0
* update ca-certificates by @jaymell in https://github.com/svix/svix-webhooks/pull/1507

## Version 1.31.0
* remove Beta tag

## Version 1.29.0
* Rebuild RabbitMQ producer on error

## Version 1.26.0
* add `/events` poller
* log svix client errors as error, not trace

## Version 1.25.0
* Upgrade omniqueue and other dependencies
* Add Kafka as an input

## Version 1.21.0
* Upgrade dependencies

## Version 1.9.0
* Switch to the Omniqueue library
* Use jemalloc as the global allocator.
* Update Docker base image and rust version.

## Version 1.8.0
* Better js transformations

## Version 1.4.12
* Initial release
