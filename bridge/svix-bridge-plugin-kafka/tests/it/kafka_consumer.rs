//! Requires a rabbitmq node to be running on localhost:5672 (the default port) and using the
//! default guest/guest credentials.
//! Try using the `testing-docker-compose.yml` in the repo root to get this going.

use std::time::Duration;

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use serde_json::json;
use svix_bridge_plugin_kafka::{KafkaConsumer, KafkaInputOpts};
use svix_bridge_types::{
    svix::api::MessageIn, CreateMessageRequest, SenderInput, SenderOutputOpts, SvixOptions,
    SvixSenderOutputOpts, TransformationConfig, TransformerInput, TransformerInputFormat,
    TransformerJob, TransformerOutput,
};
use tracing::info;
use wiremock::{
    matchers::{body_partial_json, method},
    Mock, MockServer, ResponseTemplate,
};

use crate::{create_topic, delete_topic, kafka_admin_client, BROKER_HOST};

#[ctor::ctor]
fn test_setup() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Output is only printed for failing tests, but still we shouldn't overload
                // the output with unnecessary info. When debugging a specific test, it's easy
                // to override this default by setting the `RUST_LOG` environment variable.
                "info,svix_bridge=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
}

/// Time to wait for the plugin to connect.
const CONNECT_WAIT_TIME: Duration = Duration::from_secs(10);
/// Time to wait for the plugin to receive a message sent by a test.
const CONSUME_WAIT_TIME: Duration = Duration::from_secs(1);

fn get_test_plugin(
    svix_url: String,
    topic: &str,
    use_transformation: Option<TransformerInputFormat>,
) -> KafkaConsumer {
    KafkaConsumer::new(
        "test".into(),
        KafkaInputOpts::Inner {
            bootstrap_brokers: BROKER_HOST.to_owned(),
            // All tests use different topics, so it's fine to have only one consumer group ID
            group_id: "svix_bridge_test_group_id".to_owned(),
            topic: topic.to_owned(),
            security_protocol: svix_bridge_plugin_kafka::KafkaSecurityProtocol::Plaintext,
            debug_contexts: None,
        },
        use_transformation.map(|format| TransformationConfig::Explicit {
            format,
            src: String::from("function handle(x) { return x; }"),
        }),
        SenderOutputOpts::Svix(SvixSenderOutputOpts {
            token: "xxxx".to_string(),
            options: Some(SvixOptions {
                server_url: Some(svix_url),
                ..Default::default()
            }),
        }),
    )
    .unwrap()
}

fn kafka_producer() -> FutureProducer {
    // create does block I/O, but we don't care in tests
    ClientConfig::new()
        .set("bootstrap.servers", BROKER_HOST)
        .create()
        .unwrap()
}

async fn publish(producer: &FutureProducer, topic: &str, payload: &[u8]) {
    info!(topic, "publishing message");
    producer
        .send(
            FutureRecord::<(), _>::to(topic).payload(payload),
            Timeout::After(Duration::from_secs(3)),
        )
        .await
        .unwrap();
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request.
#[tokio::test]
async fn test_consume_ok() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            "_SVIX_APP_ID": "app_1234",
            "_SVIX_EVENT_TYPE": "testing.things",
            "hi": "there",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), topic, None);

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });
    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
    };

    publish(&producer, topic, &serde_json::to_vec(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;

    handle.abort();
    delete_topic(&admin_client, topic).await;
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_json_ok() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .and(body_partial_json(json!({ "payload": { "good": "bye" } })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            // The adjustment made via the transformation...
            "good": "bye",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let mut plugin = get_test_plugin(mock_server.uri(), topic, Some(TransformerInputFormat::Json));
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let mut out = match x.input {
                TransformerInput::Json(input) => input.as_object().unwrap().clone(),
                _ => unreachable!(),
            };
            // Prune out the "hi" key.
            out["message"]["payload"]
                .as_object_mut()
                .unwrap()
                .remove("hi");
            // Add the "good" key.
            out["message"]["payload"]["good"] = json!("bye");
            x.callback_tx.send(Ok(TransformerOutput::Object(out))).ok();
        }
    });
    plugin.set_transformer(Some(transformer_tx));

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });
    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
    };

    publish(&producer, topic, &serde_json::to_vec(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;

    handle.abort();
    delete_topic(&admin_client, topic).await;
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_string_ok() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .and(body_partial_json(
            json!({ "payload": { "hello": "world" } }),
        ))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            // The adjustment made via the transformation...
            "hello": "world",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let mut plugin = get_test_plugin(
        mock_server.uri(),
        topic,
        Some(TransformerInputFormat::String),
    );
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let input = match x.input {
                TransformerInput::String(input) => input,
                _ => unreachable!(),
            };
            // Build a create-message-compatible object, using the string input as a field in the payload.
            let out = json!({
                "appId": "app_1234",
                "message": {
                    "eventType": "testing.things",
                    "payload": {
                        "hello": input,
                    }
                }
            });
            x.callback_tx
                .send(Ok(TransformerOutput::Object(
                    out.as_object().unwrap().clone(),
                )))
                .ok();
        }
    });
    plugin.set_transformer(Some(transformer_tx));

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });
    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    publish(&producer, topic, b"world").await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;

    handle.abort();
    delete_topic(&admin_client, topic).await;
}

#[tokio::test]
async fn test_missing_app_id_nack() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), topic, None);

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });

    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    publish(
        &producer,
        topic,
        &serde_json::to_vec(&json!({
            // No app id
            "message": {
                "eventType": "testing.things",
                "payload": {
                    "hi": "there",
                }
            },

        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;
    handle.abort();
    delete_topic(&admin_client, topic).await;
}

#[tokio::test]
async fn test_missing_event_type_nack() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), topic, None);

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });

    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    publish(
        &producer,
        topic,
        &serde_json::to_vec(&json!({
            "appId": "app_1234",
            "message": {
                // No event type
                "payload": {
                    "hi": "there",
                }
            },
        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;
    handle.abort();
    delete_topic(&admin_client, topic).await;
}

/// Check that the plugin keeps running when it can't send a message to svix
#[tokio::test]
async fn test_consume_svix_503() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .named("create_message")
        // The svix sdk has automatic retries, and the kafka receiver also
        // does its own retries (3 each at time of writing).
        .expect(3 * 3);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), topic, None);

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });
    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    publish(
        &producer,
        topic,
        &serde_json::to_vec(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;

    assert!(!handle.is_finished());
    handle.abort();
    delete_topic(&admin_client, topic).await;
}

/// Check that the plugin keeps running when it can't send a message to svix because idk, the servers are all offline??
#[tokio::test]
async fn test_consume_svix_offline() {
    let topic = unique_topic_name!();

    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    let producer = kafka_producer();

    let mock_server = MockServer::start().await;

    let plugin = get_test_plugin(mock_server.uri(), topic, None);

    // bye-bye svix...
    drop(mock_server);

    let handle = tokio::spawn(async move {
        plugin.run().await;
    });
    // Wait for the consumer to connect
    tokio::time::sleep(CONNECT_WAIT_TIME).await;

    publish(
        &producer,
        topic,
        &serde_json::to_vec(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(CONSUME_WAIT_TIME).await;

    assert!(!handle.is_finished());
    handle.abort();
    delete_topic(&admin_client, topic).await;
}
