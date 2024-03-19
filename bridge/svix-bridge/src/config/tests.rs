use std::collections::HashMap;

use svix_bridge_plugin_queue::config::{QueueConsumerConfig, RabbitMqInputOpts, SenderInputOpts};
use svix_bridge_types::{SenderOutputOpts, SvixSenderOutputOpts};

use super::Config;
use crate::config::{LogFormat, LogLevel, SenderConfig};

/// This is meant to be a kitchen sink config, hitting as many possible
/// configuration options as possible to ensure they parse correctly.
// FIXME: today, largely based on the examples. Should instead focus on coverage.
const OMNIBUS: &str = r#"
# Svix Bridge Example Configuration

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
          appId: input.key,
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
          appId: input.key,
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
          appId: input.key,
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
          appId: input.key,
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
    handler = (x) => ({ appId: "app_1234", message: { eventType: "foo.bar", payload: x }})
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
  # Implicit json transformation
  transformation: |
    handler = (x) => ({ appId: "app_1234", message: { eventType: "foo.bar", payload: x }})
  output:
    type: "svix"
    token: "XXXX"
- name: "from-SQS-to-svix"
  input:
    type: "sqs"
    queue_dsn: "http://sqs.example.com/foo/bar"
  # Explicit string transformation
  transformation:
    format: string
    src: |
        function handler(x) {
            return { appId: "app_1234", message: { eventType: "foo.bar", payload: x }}
        }
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

#[test]
fn test_variable_substitution_missing_vars() {
    let src = r#"
    opentelemetry: 
        address: "${OTEL_ADDR}"
    "#;
    let vars = HashMap::new();
    let cfg = Config::from_src(src, Some(&vars)).unwrap();
    let otel = cfg.opentelemetry.unwrap();
    // when lookups in the vars map fail, the original token text is preserved.
    assert_eq!(&otel.address, "${OTEL_ADDR}");
}

#[test]
fn test_variable_substitution_available_vars() {
    let src = r#"
    opentelemetry:
        address: "${OTEL_ADDR}"
        sample_ratio: ${OTEL_SAMPLE_RATIO}
    "#;
    let mut vars = HashMap::new();
    vars.insert(
        String::from("OTEL_ADDR"),
        String::from("http://127.0.0.1:8080"),
    );
    vars.insert(String::from("OTEL_SAMPLE_RATIO"), String::from("0.25"));
    let cfg = Config::from_src(src, Some(&vars)).unwrap();
    // when lookups succeed, the token should be replaced.
    let otel = cfg.opentelemetry.unwrap();
    assert_eq!(&otel.address, "http://127.0.0.1:8080");
    assert_eq!(otel.sample_ratio, Some(0.25));
}

#[test]
fn test_variable_substitution_braces_optional() {
    let src = r#"
    opentelemetry:
        # Formerly failing to use ${} notation means the port number would not be substituted.
        # Today, it works. Test that it continues to.
        address: "${OTEL_SCHEME}://${OTEL_HOST}:$OTEL_PORT"
    "#;
    let mut vars = HashMap::new();
    vars.insert(String::from("OTEL_SCHEME"), String::from("https"));
    vars.insert(String::from("OTEL_HOST"), String::from("127.0.0.1"));
    vars.insert(String::from("OTEL_PORT"), String::from("9999"));
    let cfg = Config::from_src(src, Some(&vars)).unwrap();
    // when lookups succeed, the token should be replaced.
    let otel = cfg.opentelemetry.unwrap();
    // Not the user-intended outcome, but it simplifies the parsing requirements.
    assert_eq!(&otel.address, "https://127.0.0.1:9999");
}

#[test]
fn test_variable_substitution_missing_numeric_var_is_err() {
    // Unfortunate side-effect of templating yaml.
    //
    // If the variable is missing, usually you've got three options:
    // - retain the token text that failed the lookup (envsubst-rs does this)
    // - replace the token with an empty string (the CLI `envsubst` does this)
    // - mark it an error (neither do this, but we can if we roll our own impl)
    //
    // For yaml, the field typings are heavily/poorly inferred so for an optional float like
    // `sample_ratio` an empty string would parse as a `None`, which could be a bad fallback since
    // otel considers this a 1.0 ratio (send everything).
    //
    // For this specific case, retaining the token text produces an error, which happens to be useful.
    // For fields that happen to be strings anyway, errors may show up later (after the config parsing).
    // Ex: using `${QUEUE_NAME}` in a rabbit sender input will surface in logs as an error when we
    // try to connect: "no such queue '${QUEUE_NAME}'".

    let src = r#"
    opentelemetry:
        address: "${OTEL_ADDR}"
        # This var will be missing, causing the template token to
        # be retained causing a parse failure :(
        sample_ratio: ${OTEL_SAMPLE_RATIO}
    "#;
    let vars = HashMap::new();
    let err = Config::from_src(src, Some(&vars)).err().unwrap();
    let want = "Failed to parse config: opentelemetry.sample_ratio: invalid type: \
                    string \"${OTEL_SAMPLE_RATIO}\", expected f64 at line 6 column 23";
    assert_eq!(want, err.to_string());
}

/// This is probably a given, but we should expect a single variable can be referenced multiple
/// times within the config.
/// The concrete use case: auth tokens.
#[test]
fn test_variable_substitution_repeated_lookups() {
    let src = r#"
    senders:
      - name: "rabbitmq-1"
        input:
          type: "rabbitmq"
          uri: "${RABBIT_URI}"
          queue_name: "${QUEUE_NAME_1}"
        output:
          type: "svix"
          token: "${SVIX_TOKEN}"
      - name: "rabbitmq-2"
        input:
          type: "rabbitmq"
          uri: "${RABBIT_URI}"
          queue_name: "${QUEUE_NAME_2}"
        output:
          type: "svix"
          token: "${SVIX_TOKEN}"
    "#;
    let mut vars = HashMap::new();
    vars.insert(
        String::from("RABBIT_URI"),
        String::from("amqp://guest:guest@localhost:5672/%2f"),
    );
    vars.insert(String::from("QUEUE_NAME_1"), String::from("one"));
    vars.insert(String::from("QUEUE_NAME_2"), String::from("two"));
    vars.insert(String::from("SVIX_TOKEN"), String::from("x"));
    let cfg = Config::from_src(src, Some(&vars)).unwrap();

    if let SenderConfig::QueueConsumer(QueueConsumerConfig {
        input:
            SenderInputOpts::RabbitMQ(RabbitMqInputOpts {
                uri, queue_name, ..
            }),
        output: SenderOutputOpts::Svix(SvixSenderOutputOpts { token, .. }),
        ..
    }) = &cfg.senders[0]
    {
        assert_eq!(uri, "amqp://guest:guest@localhost:5672/%2f");
        assert_eq!(queue_name, "one");
        assert_eq!(token, "x");
    } else {
        panic!("sender did not match expected pattern");
    }

    if let SenderConfig::QueueConsumer(QueueConsumerConfig {
        input:
            SenderInputOpts::RabbitMQ(RabbitMqInputOpts {
                uri, queue_name, ..
            }),
        output: SenderOutputOpts::Svix(SvixSenderOutputOpts { token, .. }),
        ..
    }) = &cfg.senders[1]
    {
        assert_eq!(uri, "amqp://guest:guest@localhost:5672/%2f");
        assert_eq!(queue_name, "two");
        assert_eq!(token, "x");
    } else {
        panic!("sender did not match expected pattern");
    }
}

/// This is to ensure the order of operations.
/// Variables in js source code fragments could result in invalid JS source.
/// Now that we are validating transformations parse as JS, this test aims to make sure the
/// replacements happen before the JS parsing does.
#[test]
fn test_variable_substitution_in_transformations() {
    let src = r#"
    senders:
      - name: "rabbitmq-1"
        input:
          type: "rabbitmq"
          uri: "${RABBIT_URI}"
          queue_name: "${QUEUE_NAME}"
        transformation: |
          function handler(input) {
            return {
                appId: "xxx",
                message: {
                  eventType: "queue.message.handled",
                  payload: {
                    queueName: "${QUEUE_NAME}",
                    // Without the substitution for NUMBER, this expression would be invalid syntax.
                    number: ${NUMBER} - 10,
                    data: input,
                  }
                }
            };
          }
        output:
          type: "svix"
          token: "${SVIX_TOKEN}"
    "#;
    let mut vars = HashMap::new();
    vars.insert(String::from("NUMBER"), String::from("123"));
    vars.insert(String::from("QUEUE_NAME"), String::from("one"));
    let cfg = Config::from_src(src, Some(&vars)).unwrap();

    let xform = cfg.senders[0].transformation().unwrap();
    xform.source().contains(r#"queueName: "one""#);
    xform.source().contains(r#"number: 123 - 10,"#);
}

/// Check that the config parser validates the JS source fragments in it.
#[test]
fn test_transformation_validation_bad_syntax_is_err() {
    let src = r#"
    senders:
      - name: "bad xform"
        input:
          type: "rabbitmq"
          uri: "xxx"
          queue_name: "xxx"
        transformation: |
          // invalid syntax
          let 123 = 456
        output:
          type: "svix"
          token: "xxx"
    "#;
    let err = Config::from_src(src, None).err().unwrap();
    assert!(err
        .to_string()
        .contains("failed to parse transformation for sender `bad xform`"))
}

#[test]
fn test_var_substitution_json_values_ok() {
    let src = r#""#;
    let mut vars = HashMap::new();
    // GCP credentials is one place where we _expect_ json to be supplied as an env var.
    // We need to be able to support this.
    vars.insert(
        "GOOGLE_APPLICATION_CREDENTIALS_JSON".into(),
        r#"{"foo": true, "bar": 123}"#.into(),
    );
    // Should not be an error
    let _cfg = Config::from_src(src, Some(&vars)).unwrap();
}
