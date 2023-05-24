# Svix Bridge

`svix-bridge` is organized in terms of **senders** and **receivers**.

**Senders** are useful when you have a data source (an "input") such as a
message queue and want to generate Svix webhooks from those messages.

**Receivers** act as HTTP endpoints which wait for Svix webhooks to arrive, then
publish the payload on to a specified "output."

Currently the supported Sender inputs and Receiver outputs are the following
messaging systems:

- GCP Pub/Sub
- RabbitMQ
- Redis
- SQS

## Usage

```
svix-bridge -c path/to/svix-bridge.yaml
```

## Configuration

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

Important to note that queues, exchanges, topics, or what have you, should be created and configured independently,
prior to using the plugin. There's nothing in place to automatically create these resources.
The plugin will only try (and fail) to read from the stream in such a case.

- GCP Pub/Sub
- RabbitMQ
- Redis
- SQS

The HTTP server also (optionally) performs validation of the webhooks using Svix's signature verification method.

The `verification` section for each route can be set one of two ways:
* `none` which accepts and forwards any JSON POST HTTP request.
* `svix` that takes a Svix endpoint secret (starting with `whsec_`) and
  validating it using an official Svix library
