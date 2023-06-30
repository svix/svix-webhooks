//! Use the `testing-docker-compose.yml` in the repo root to run the dependencies for testing,
//! including the gcloud pubsub emulator.
//!
//! Use `run-tests.sh` to use the requisite environment for testing.

use google_cloud_googleapis::pubsub::v1::{DeadLetterPolicy, PubsubMessage};
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::{Subscription, SubscriptionConfig};
use google_cloud_pubsub::topic::Topic;
use std::time::Duration;

use serde_json::json;
use svix_bridge_plugin_queue::{config::GCPPubSubInputOpts, GCPPubSubConsumerPlugin};
use svix_bridge_types::{
    svix::api::MessageIn, CreateMessageRequest, SenderInput, SenderOutputOpts, SvixOptions,
    SvixSenderOutputOpts, TransformationConfig, TransformerInput, TransformerInputFormat,
    TransformerJob, TransformerOutput,
};
use wiremock::matchers::{body_partial_json, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

const DEFAULT_PUBSUB_EMULATOR_HOST: &str = "localhost:8085";

fn get_test_plugin(
    svix_url: String,
    subscription_id: String,
    use_transformation: Option<TransformerInputFormat>,
) -> GCPPubSubConsumerPlugin {
    GCPPubSubConsumerPlugin::new(
        "test".into(),
        GCPPubSubInputOpts {
            subscription_id,
            credentials_file: None,
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
}

async fn mq_connection() -> Client {
    // The `Default` impl for `ClientConfig` looks for this env var. When set it branches for
    // local-mode use using the addr in the env var and a hardcoded project id of `local-project`.
    if std::env::var("PUBSUB_EMULATOR_HOST").is_err() {
        std::env::set_var("PUBSUB_EMULATOR_HOST", DEFAULT_PUBSUB_EMULATOR_HOST);
    }
    Client::new(ClientConfig::default()).await.unwrap()
}

fn random_chars() -> impl Iterator<Item = char> {
    std::iter::repeat_with(fastrand::alphanumeric)
}

async fn create_test_queue(client: &Client) -> (Topic, Subscription) {
    let topic_name: String = "topic-".chars().chain(random_chars().take(8)).collect();
    // Need to define a dead letter topic to avoid the "bad" test cases from pulling the nacked
    // messages again and again.
    let dead_letter_topic_name: String = "topic-".chars().chain(random_chars().take(8)).collect();
    let subscription_name: String = "subscription-"
        .chars()
        .chain(random_chars().take(8))
        .collect();

    let topic = client.create_topic(&topic_name, None, None).await.unwrap();
    let dead_letter_topic = client
        .create_topic(&dead_letter_topic_name, None, None)
        .await
        .unwrap();
    let subscription = client
        .create_subscription(
            &subscription_name,
            &topic_name,
            SubscriptionConfig {
                // Messages published to the topic need to supply a unique ID to make use of this
                enable_exactly_once_delivery: true,
                dead_letter_policy: Some(DeadLetterPolicy {
                    dead_letter_topic: dead_letter_topic.fully_qualified_name().into(),
                    max_delivery_attempts: MAX_DELIVERY_ATTEMPTS,
                }),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    (topic, subscription)
}

async fn publish(topic: &Topic, payload: &str) {
    let publisher = topic.new_publisher(None);
    let awaiter = publisher
        .publish(PubsubMessage {
            data: payload.to_owned().into_bytes(),
            message_id: random_chars().take(6).collect(),
            ..Default::default()
        })
        .await;
    awaiter.get().await.unwrap();
}

/// General "pause while we wait for messages to travel" beat. If you're seeing flakes, bump this up.
const WAIT_MS: u64 = 100;
/// Controls how many times a message can be nack'd before it lands on the dead letter topic.
const MAX_DELIVERY_ATTEMPTS: i32 = 5;

/// Push a msg on the queue.
/// Check to see if the svix server sees a request.
#[tokio::test]
async fn test_consume_ok() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

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

    let plugin = get_test_plugin(mock_server.uri(), subscription.id(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&topic, &serde_json::to_string(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_json_ok() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        // The transformed bit of the payload
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

    let mut plugin = get_test_plugin(
        mock_server.uri(),
        subscription.id(),
        Some(TransformerInputFormat::Json),
    );
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let mut out = match x.input {
                TransformerInput::JSON(input) => input.as_object().unwrap().clone(),
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
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&topic, &serde_json::to_string(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_string_ok() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        // The transformed bit of the payload
        .and(body_partial_json(
            json!({ "payload": { "hello": "world" } }),
        ))
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

    let mut plugin = get_test_plugin(
        mock_server.uri(),
        subscription.id(),
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
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(&topic, "world").await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

#[tokio::test]
async fn test_missing_app_id_nack() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), subscription.id(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &topic,
        &serde_json::to_string(&json!({
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
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

#[tokio::test]
async fn test_missing_event_type_nack() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), subscription.id(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &topic,
        &serde_json::to_string(&json!({
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
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

/// Check that the plugin keeps running when it can't send a message to svix
#[tokio::test]
async fn test_consume_svix_503() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .named("create_message")
        // N.b. this test case is different than other backend flavors of these since there's a
        // minimum of 5 delivery attempts made before messages are forwarded to the dead letter topic.
        // In other cases this can happen immediately, but not with gcp pubsub.
        .up_to_n_times(MAX_DELIVERY_ATTEMPTS.try_into().unwrap())
        .expect(1..);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), subscription.id(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &topic,
        &serde_json::to_string(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}

/// Check that the plugin keeps running when it can't send a message to svix because idk, the servers are all offline??
#[tokio::test]
async fn test_consume_svix_offline() {
    let client = mq_connection().await;
    let (topic, subscription) = create_test_queue(&client).await;

    let mock_server = MockServer::start().await;

    let plugin = get_test_plugin(mock_server.uri(), subscription.id(), None);

    // bye-bye svix...
    drop(mock_server);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &topic,
        &serde_json::to_string(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();

    subscription.delete(None).await.ok();
    topic.delete(None).await.ok();
}
