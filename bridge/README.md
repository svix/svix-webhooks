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

# Svix Bridge (beta)

Bridge is an agent to help integrate webhooks into your existing messaging infrastructure.

Bridge is organized in terms of **senders** and **receivers**.

**Senders** are useful when you have a data source (an "input") such as a
message queue and want to generate Svix webhooks from those messages.

**Receivers** act as HTTP endpoints which wait for Svix webhooks to arrive, then
publish the payload on to a specified "output."

**Receivers** also (optionally) perform validation of the webhooks using Svix's signature verification.

Both **senders** and **receivers** are defined in terms of their input, and optional JavaScript transformation, and their output.

---

Transformations are configured as either a single string of JS source code:

```yaml
transformation: |
    function handler(input) {
      return {
        appId: input.key,
        message: {
          eventType: input.event_type,
          payload: input.data
        }
      };
    }
```
In this situation, the input value will be pre-parsed as JSON.

You can also configure transformations using the verbose form, allowing control of the `format`:
```yaml
transformation:
  format: "json"
  src: |
    function handler(input) {
      return {
        appId: input.key,
        message: {
          eventType: input.event_type,
          payload: input.data
        }
      };
    }
```
The `format` field can be set to either `string` (ie. the _raw payload_) or `json` (the default behavior).

For example, using the string `format` you can parse data out of the input using whatever method you like:

```yaml
transformation:
  # Let's say the input is an XML message like:
  # `<msg appId="app_123" eventType="a.b"><payload>{"x": 123}</payload></msg>`

  format: "string"
  src: |
    function handler(input) {
      let parser = new DOMParser();
      let doc = parser.parseFromString(input, "text/xml");
      let msg = doc.firstChild;
      let payload = JSON.parse(msg.getElementsByTagName("payload")[0].textContent)

      return {
        appId: msg.attributes.appId,
        message: {
          eventType: msg.attributes.eventType,
          payload,
        }
      };
    }
```
Transformations must define a function named `handler` which receives a single argument, the type of which is controlled
by the configured `format` field.

Note that regardless of the `format`, the return type of `handler` must be an `Object`.

> N.b. at time of writing, `format: string` is unsupported for `senders` and `receivers` configured with
> a `redis` input or output.

---

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

## Variable Expansion

`svix-bridge` supports environment variable expansion inside the config file.

Using "dollar brace" notation, e.g. `${MY_VARIABLE}`, a substitution will be made from the environment.
If the variable lookup fails, the raw variable text is left in the config as-is.

As an example, here's a RabbitMQ sender configured with environment variables:

```yaml
senders:
  - name: "send-webhooks-from-${QUEUE_NAME}"
    input:
      type: "rabbitmq"
      uri: "${MQ_URI}"
      queue_name: "${QUEUE_NAME}"
    output:
      type: "svix"
      token: "${SVIX_TOKEN}"
```

Each sender and receiver can optionally specify a `transformation`.
Transformations should define a function called `handler` that accepts an object and returns an object.

Senders should produce JSON following an expected shape:
```json
{
    "appId": "app_XYZ",
    "message": {
        "eventType": "my.event",
        "payload": {"code": 123, "message": "something happened..."}
    }
}
```

For detail on the `message` field, see: <https://api.svix.com/docs#tag/Message/operation/v1.message.create>

Receivers can accept arbitrary body data but the outputs require a JSON object with a `payload` field representing the
message to publish.

```json
{
    "payload": {"msg": "my cool message, published by svix-bridge!"}
}
```

When a transformation is _not configured_, the default behavior is to parse the request body as JSON and set this value
as the `payload` field before forwarding to the output.

By configuring a transformation, you should be able to consume a variety of `POST` bodies and
produce a valid output, but just remember to make sure the _return value_ has your data attached to the `payload` field.


See the example configs for how to configure each input and output in more detail:
- [senders](./svix-bridge.example.senders.yaml)
- [receivers](./svix-bridge.example.receivers.yaml)

# Building from source

You would need a working Rust compiler in order to build Svix Bridge.
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
