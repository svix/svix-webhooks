//! Use the `testing-docker-compose.yml` in the repo root to run the dependencies for testing,
//! including ElasticMQ.
//!
//! Use `run-tests.sh` to use the requisite environment for testing.

use std::time::Duration;

use aws_sdk_sqs::Client;
use serde_json::json;
use svix::api::MessageIn;
use svix_agent_plugin_generic::{
    config::{OutputOpts, SvixOptions},
    CreateMessageRequest, SqsConsumerConfig, SqsConsumerPlugin, SqsInputOpts,
};
use svix_agent_types::Plugin;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

const ROOT_URL: &str = "http://localhost:9324";

fn get_test_plugin(svix_url: String, queue_dsn: String) -> SqsConsumerPlugin {
    SqsConsumerPlugin::new(SqsConsumerConfig {
        input: SqsInputOpts {
            queue_dsn,
            override_endpoint: true,
        },
        output: OutputOpts {
            token: "xxxx".to_string(),
            svix_options: Some(SvixOptions {
                server_url: Some(svix_url),
                ..Default::default()
            }),
        },
    })
}

async fn mq_connection() -> Client {
    let config = aws_config::from_env().endpoint_url(ROOT_URL).load().await;
    Client::new(&config)
}

async fn create_test_queue(client: &Client) -> String {
    let name: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(8)
        .collect();
    client
        .create_queue()
        .queue_name(&name)
        .send()
        .await
        .unwrap();

    name
}

async fn publish(client: &Client, url: &str, payload: &str) {
    client
        .send_message()
        .queue_url(url)
        .message_body(payload)
        .send()
        .await
        .unwrap();
}

/// General "pause while we wait for messages to travel" beat. If you're seeing flakes, bump this up.
const WAIT_MS: u64 = 100;

/// Push a msg on the queue.
/// Check to see if the svix server sees a request.
#[tokio::test]
async fn test_consume_ok() {
    let client = mq_connection().await;
    let queue_name = create_test_queue(&client).await;

    let queue_url = format!("{ROOT_URL}/queue/{queue_name}");

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

    let plugin = get_test_plugin(mock_server.uri(), queue_url.clone());

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

    publish(&client, &queue_url, &serde_json::to_string(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_missing_app_id_nack() {
    let client = mq_connection().await;
    let queue_name = create_test_queue(&client).await;

    let queue_url = format!("{ROOT_URL}/queue/{queue_name}");

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), queue_url.clone());

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &queue_url,
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

    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_missing_event_type_nack() {
    let client = mq_connection().await;
    let queue_name = create_test_queue(&client).await;

    let queue_url = format!("{ROOT_URL}/queue/{queue_name}");

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), queue_url.clone());

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &queue_url,
        &serde_json::to_string(&json!({
            "app_id": "app_1234",
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

    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
}

/// Check that the plugin keeps running when it can't send a message to svix
#[tokio::test]
async fn test_consume_svix_503() {
    let client = mq_connection().await;
    let queue_name = create_test_queue(&client).await;

    let queue_url = format!("{ROOT_URL}/queue/{queue_name}");

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), queue_url.clone());

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &queue_url,
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

    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
}

/// Check that the plugin keeps running when it can't send a message to svix because idk, the servers are all offline??
#[tokio::test]
async fn test_consume_svix_offline() {
    let client = mq_connection().await;
    let queue_name = create_test_queue(&client).await;

    let queue_url = format!("{ROOT_URL}/queue/{queue_name}");

    let mock_server = MockServer::start().await;

    let plugin = get_test_plugin(mock_server.uri(), queue_url.clone());

    // bye-bye svix...
    drop(mock_server);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &queue_url,
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

    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
}
