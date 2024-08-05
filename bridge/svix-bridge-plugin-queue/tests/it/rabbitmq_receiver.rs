//! Requires a rabbitmq node to be running on localhost:5672 (the default port) and using the
//! default guest/guest credentials.
//! Try using the `testing-docker-compose.yml` in the repo root to get this going.

use std::time::Duration;

use lapin::{
    options::QueueDeclareOptions, types::FieldTable, Channel, Connection, ConnectionProperties,
    Queue,
};
use serde_json::json;
use svix_bridge_plugin_queue::config::{QueueForwarder, QueueOutputOpts, RabbitMqOutputOpts};
use svix_bridge_types::{ForwardRequest, ReceiverOutput};
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task::JoinHandle,
};

async fn declare_queue(name: &str, channel: &Channel) -> Queue {
    channel
        .queue_declare(
            name,
            QueueDeclareOptions {
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap()
}

async fn mq_connection(uri: &str) -> Connection {
    let options = ConnectionProperties::default()
        .with_connection_name("test".into())
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    Connection::connect(uri, options).await.unwrap()
}

const WAIT_MS: u64 = 200;

/// These tests assume a "vanilla" rabbitmq instance, using the default port, creds, exchange...
const MQ_URI: &str = "amqp://guest:guest@localhost:5672/%2f";

/// TCP proxy. Useful for giving us control over the connection to rabbit inside our tests.
async fn proxy(
    listener: TcpListener,
    server_addr: impl ToSocketAddrs + Clone + Sync + Send + 'static,
) -> Result<JoinHandle<()>, ()> {
    let handle = tokio::task::spawn(async move {
        while let Ok((mut inbound, _)) = listener.accept().await {
            let mut outbound = TcpStream::connect(server_addr.clone()).await.unwrap();
            if let Err(e) = copy_bidirectional(&mut inbound, &mut outbound).await {
                eprintln!("Failed to transfer; error={e}");
            }
        }
    });
    Ok(handle)
}

#[tokio::test]
async fn test_connection_recovery() {
    let mq_conn = mq_connection(MQ_URI).await;
    let channel = mq_conn.create_channel().await.unwrap();
    // setup the queue before running the consumer or the consumer will error out
    let queue = declare_queue("", &channel).await;
    let queue_name = queue.name().as_str();

    let proxy_listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let port = proxy_listener.local_addr().unwrap().port();
    // Start the proxy
    let proxy_handle = proxy(proxy_listener, "127.0.0.0:5672").await.unwrap();

    // Configure the receiver output to connect to the proxy so we can interrupt the connection as needed.
    let proxied_mq_uri = format!("amqp://guest:guest@localhost:{port}/%2f");

    let opts = QueueOutputOpts::RabbitMQ(RabbitMqOutputOpts {
        uri: proxied_mq_uri.clone(),
        exchange: "".to_string(),
        routing_key: queue_name.to_string(),
        publish_options: Default::default(),
        publish_properties: Default::default(),
    });

    let output = QueueForwarder::from_receiver_output_opts(String::from("test"), opts)
        .await
        .unwrap();

    let req = ForwardRequest {
        payload: json!({"test": true}),
    };

    assert!(
        output.handle(req.clone()).await.is_ok(),
        "expected ok when rabbit available"
    );

    // Disconnect the proxy
    proxy_handle.abort();
    // Sleep a beat to give time for the proxy tear down.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(
        output.handle(req.clone()).await.is_err(),
        "expected err when rabbit unavailable"
    );

    // Reconnect the proxy on the same port
    let proxy_listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    let proxy_handle = proxy(proxy_listener, "127.0.0.0:5672").await.unwrap();

    assert!(
        output.handle(req.clone()).await.is_ok(),
        "expected ok when rabbit available"
    );

    proxy_handle.abort();
}
