use super::Config;
use crate::config::{LogFormat, LogLevel, SenderConfig};

/// This is meant to be a kitchen sink config, hitting as many possible
/// configuration options as possible to ensure they parse correctly.
// FIXME: today, largely based on the examples. Should instead focus on coverage.
const OMNIBUS: &str = r#"
# Svix Webhook Bridge Example Configuration

# Set the log level for the service. Supported: error, info, warn, debug, trace. Default: info
#log_level: "debug"

# The log format that all output will follow. Supported: default, json
#log_format: "json"

# Optional: configures an OTEL exporter forwarding spans to the specified collector
opentelemetry:
  address: "http://localhost:1234"
  sample_ratio: 0.5
  # Optional: default "svix-bridge"
  service_name: "my-bridge"


# The host/port to bind to for incoming HTTP requests.
# Optional: default 0.0.0.0:5000
# http_listen_address: "0.0.0.0:5000"


# Senders consume JSON from their input, optionally transform, then forward to Svix.
# Currently supported inputs are various message queue consumers while the sole
# output is "svix" (which does a Create Message API request)
senders:
  # GCP Pub/Sub Consumer
  - name: "gcp-example"
    input:
      type: "gcp-pubsub"
      subscription_id: "my-subscription"
      # Optional - will fallback to looking at env vars when left unset.
      credentials_file: "/path/to/credentials.json"
    # Optional - when unset, messages from the queue will be sent to Svix as-is.
    transformation: |
      function handler(input) {
        return {
          app_id: input.key,
          message: {
            eventType: input.event_type,
            payload: input.data
          }
        };
      }
    output:
      type: "svix"
      # Required (the Svix token to use when creating messages with this consumer)
      token: "XYZ"

  # RabbitMQ Consumer
  - name: "rabbitmq-example"
    input:
      type: "rabbitmq"
      # Required
      uri: "amqp://guest:guest@localhost:5672/%2f"
      # Required
      queue_name: "my-queue"
      # Optional (default: unset, managed by rabbitmq)
      consumer_tag: "my-consumer-001"
      # Optional: default true
      requeue_on_nack: false
    # Optional - when unset, messages from the queue will be sent to Svix as-is.
    transformation: |
      function handler(input) {
        return {
          app_id: input.key,
          message: {
            eventType: input.event_type,
            payload: input.data
          }
        };
      }
    output:
      type: "svix"
      # Required (the Svix token to use when creating messages with this consumer)
      token: "XYZ"

  # Redis Consumer
  - name: "redis-example"
    input:
      type: "redis"
      # Required
      dsn: "redis://localhost:6379/"
      # Required
      queue_key: "my_queue"
      # Required
      consumer_name: "my_consumer"
      # Required
      consumer_group: "my_group"
      # Required
      max_connections: 4
      # Optional: default true
      reinsert_on_nack: true
    # Optional - when unset, messages from the queue will be sent to Svix as-is.
    transformation: |
      function handler(input) {
        return {
          app_id: input.key,
          message: {
            eventType: input.event_type,
            payload: input.data
          }
        };
      }
    output:
      type: "svix"
      # Required (the Svix token to use when creating messages with this consumer)
      token: "XYZ"

  # SQS Consumer
  # Also remember to set your AWS credentials in env vars to use this:
  # - `AWS_DEFAULT_REGION`
  # - `AWS_ACCESS_KEY_ID`
  # - `AWS_SECRET_ACCESS_KEY`
  - name: "sqs-example"
    input:
      type: "sqs"
      # Required
      queue_dsn: "http://localhost:19324/000000000000/local"
      # Optional (default: false)
      override_endpoint: true
    # Optional - when unset, messages from the queue will be sent to Svix as-is.
    transformation: |
      function handler(input) {
        return {
          app_id: input.key,
          message: {
            eventType: input.event_type,
            payload: input.data
          }
        };
      }
    output:
      type: "svix"
      # Required (the Svix token to use when creating messages with this consumer)
      token: "XYZ"


# Receivers are HTTP endpoints that can have webhooks sent to them.
# When a webhook is POST'ed to a matching URL, it is (optionally) verified,
# (optionally) transformed via a js function, then forwarded to an "output."
#
# Inputs types are "webhook" which allows you to configure a verification scheme
# (either "svix" or "none") or "svix-webhook" which is a shorthand version.
#
# ```
#   input:
#     type: "webhook"
#     path_id: "long-hand"
#     verification:
#       type: "svix"
#       endpoint_secret: "whsec_XXXXX="
#   # same as...
#   input:
#     type: "svix-webhook"
#     path_id: "shorthand"
#     endpoint_secret: "whsec_XXXXX="
# ```
#
# The `path_id` in webhook and svix-webhook inputs represents the trailing
# path segment that will connect to the given output.
# For example, running bridge with the HTTP listen address set to
# `localhost:5000`, the above examples would map to the following URLS:
# - http://localhost:5000/webhooks/long-hand
# - http://localhost:5000/webhooks/shorthand
#
receivers:
  - name: "forward-to-gcp-example"
    input:
      type: "webhook"
      path_id: "gcp"
      verification:
        type: "svix"
        endpoint_secret: "whsec_XXXXX="
    # Optional - when unset, webhooks received will be forwarded to the output as-is.
    transformation: |
      function handler(input) {
        let event_type = input.eventType;
        delete input.eventType;
        return { event_type, ...input };
      }
    output:
      type: "gcp-pubsub"
      topic: "example"
      # Optional - falls back to env otherwise, eg.
      # - `GOOGLE_APPLICATION_CREDENTIALS`
      # - `GOOGLE_APPLICATION_CREDENTIALS_JSON`
      credentials_file: "/path/to/creds.json"

  - name: "forward-to-rabbitmq-example"
    input:
      type: "webhook"
      path_id: "rabbit"
      verification:
        type: "svix"
        endpoint_secret: "whsec_XXXXX="
    output:
      type: "rabbitmq"
      uri: "amqp://guest:guest@localhost:5672/%2f"
      exchange: ""
      routing_key: "example"

  - name: "forward-to-redis-example"
    input:
      type: "webhook"
      path_id: "redis"
      verification:
        type: "svix"
        endpoint_secret: "whsec_XXXXX="
    output:
      type: "redis"
      dsn: "redis://localhost:1234"
      max_connections: 4
      queue_key: "my_queue"

  - name: "forward-to-sqs-example"
    input:
      type: "webhook"
      path_id: "sqs"
      verification:
        type: "none"
    output:
      # Note that the SQS forwarder requires credentials to be set as environment vars:
      # - `AWS_DEFAULT_REGION`
      # - `AWS_ACCESS_KEY_ID`
      # - `AWS_SECRET_ACCESS_KEY`
      type: "sqs"
      queue_dsn: "https://example.aws.com/my-queue"
"#;

#[test]
fn test_sender_parses_ok() {
    let conf: Result<SenderConfig, _> = serde_yaml::from_str(
        r#"
name: "from-rabbit-local-to-svix"
input:
    type: "rabbitmq"
    queue_name: "local"
    uri: "amqp://example.com/%2f"
transformation: |
    handler = (x) => ({ app_id: "app_1234", message: { eventType: "foo.bar", payload: x }})
output:
    type: "svix"
    token: "XXXX"
    "#,
    );
    conf.unwrap();
}

#[test]
fn test_senders_parses_ok() {
    let conf: Result<Vec<SenderConfig>, _> = serde_yaml::from_str(
        r#"

- name: "from-rabbit-local-to-svix"
  input:
    type: "rabbitmq"
    queue_name: "local"
    uri: "amqp://example.com/%2f"
  transformation: |
    handler = (x) => ({ app_id: "app_1234", message: { eventType: "foo.bar", payload: x }})
  output:
    type: "svix"
    token: "XXXX"
- name: "from-SQS-to-svix"
  input:
    type: "sqs"
    queue_dsn: "http://sqs.example.com/foo/bar"
  transformation: |
    handler = (x) => ({ app_id: "app_1234", message: { eventType: "foo.bar", payload: x }})
  output:
    type: "svix"
    token: "YYYY"
"#,
    );
    let conf = conf.unwrap();
    assert_eq!(conf.len(), 2);
}

#[test]
fn test_omnibus_parses_ok() {
    let conf: Result<Config, _> = serde_yaml::from_str(OMNIBUS);
    conf.unwrap();
}

#[test]
fn test_empty() {
    let conf: Config = serde_yaml::from_str("").unwrap();
    assert!(conf.senders.is_empty());
    assert!(conf.receivers.is_empty());
    assert_eq!(conf.http_listen_address, "0.0.0.0:5000".parse().unwrap());
    assert!(conf.opentelemetry.is_none());
    assert!(matches!(conf.log_format, LogFormat::Default));
    assert!(matches!(conf.log_level, LogLevel::Info));
}

/// Don't particularly care about the parsed specifics here.
/// This is more about making sure the examples we have in the repo actually parse.
#[test]
fn test_receivers_example() {
    let fp = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../svix-bridge.example.receivers.yaml"
    );
    let conf: Config = serde_yaml::from_slice(&std::fs::read(fp).unwrap()).unwrap();
    assert!(conf.senders.is_empty());
    assert!(!conf.receivers.is_empty());
}

/// Don't particularly care about the parsed specifics here.
/// This is more about making sure the examples we have in the repo actually parse.
#[test]
fn test_senders_example() {
    let fp = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../svix-bridge.example.senders.yaml"
    );
    let conf: Config = serde_yaml::from_slice(&std::fs::read(fp).unwrap()).unwrap();
    assert!(!conf.senders.is_empty());
    assert!(conf.receivers.is_empty());
}
