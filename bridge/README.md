<h1 align="center">
  <a href="https://www.svix.com">
    <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
    <p align="center">Svix - Webhooks as a service</p>
  </a>
</h1>

![GitHub tag](https://img.shields.io/github/tag/svix/svix-webhooks.svg)
[![Build Status](https://github.com/svix/svix-webhooks/workflows/Bridge%20CI/badge.svg)](https://github.com/svix/svix-webhooks/actions)
[![Bridge Security](https://github.com/svix/svix-webhooks/actions/workflows/bridge-security.yml/badge.svg)](https://github.com/svix/svix-webhooks/actions/workflows/bridge-security.yml)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

# Svix Bridge

Bridge is an agent to help integrate webhooks into your existing messaging infrastructure.

Bridge is organized in terms of **senders** and **receivers**.

**Senders** are useful when you have a data source (an "input") such as a
message queue and want to generate Svix webhooks from those messages.

**Receivers** act as HTTP endpoints which wait for Svix webhooks to arrive, then
publish the payload on to a specified "output."

**Receivers** also (optionally) perform validation of the webhooks using Svix's signature verification.

Both **senders** and **receivers** are defined in terms of their input, and optional JavaScript transformation, and their output.

Currently the supported Sender inputs and Receiver outputs are the following
messaging systems:

- GCP Pub/Sub
- RabbitMQ
- Redis
- SQS

> Important to note that queues, exchanges, topics, etc should be created and configured independently,
> prior to using launching Bridge. Bridge will not automatically attempt to create these resources, it will only try
> (and fail) to read from or publish to the stream/queue in this case.


## Installation

Docker images are available on [docker hub](https://registry.hub.docker.com/r/svix/svix-bridge)

```
$ docker pull svix/svix-bridge
```

If you don't want to use docker, see [Building from Source](../README.md#building-from-source).



# Usage and Configuration

```
$ svix-bridge -c path/to/svix-bridge.yaml
```

The CLI itself exposes only a single flag (`-c`, `--cfg`) used to set the path for the config file.
The location of the config file can also be set with the `SVIX_BRIDGE_CFG` env var.
The config file itself does the heavy lifting.

When unset, the current working directory is checked for a file named `svix-bridge.yaml`.

Each sender and receiver can optionally specify a `transformation`.
Transformations should define a function called `handler` that accepts an object and returns an object.

Senders should produce JSON following an expected shape:
```
{
    // This indicates which Svix application to send the message to
    "app_id": "app_XYZ",
    
    // The `message` field has the same requirements as the standard `MessageIn`
    // used for Create Message API requests
    "message": {
        "eventType": "my.event",
        "payload": {"abc": 123}
    }
}
```

> The comments in the above JSON are for illustrative purposes only ;)
> That's not valid JSON! Sorry!

For detail on the `message` field, see: <https://api.svix.com/docs#tag/Message/operation/v1.message.create>

See the example configs for how to configure each input and output in more detail:
- [senders](./svix-bridge.example.senders.yaml)
- [receivers](./svix-bridge.example.receivers.yaml)

# Building from source

You would need a working Rust complier in order to build Svix Bridge.
The easiest way is to use [rustup](https://rustup.rs/).

```
# Clone the repository
git clone https://github.com/svix/svix-webhooks
# Change to the source directory
cd svix-webhooks/bridge/
# Build
cargo install --path svix-bridge
```

Some system dependencies are required for Bridge to build successfully.
Consult the [Dockerfile](./Dockerfile) for a good reference of what's required at build time.

# Building with Docker

```
# Clone the repository
git clone https://github.com/svix/svix-webhooks
# Change to the source directory
cd svix-webhooks/bridge/
# Build
docker build --tag svix-bridge:local .
```
