use std::{sync::Arc, time::Duration};

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use serde_json::json;
use svix_bridge_plugin_kafka::{KafkaOutputOpts, KafkaProducer};
use svix_bridge_types::{ForwardRequest, ReceiverOutput as _};

use crate::{create_topic, delete_topic, kafka_admin_client, BROKER_HOST};

/// Time to wait for the consumer to be properly listening.
const LISTEN_WAIT_TIME: Duration = Duration::from_secs(8);

#[tokio::test]
async fn test_produce_ok() {
    let topic = unique_topic_name!();
    let admin_client = kafka_admin_client();
    create_topic(&admin_client, topic).await;

    // Start listening for messages
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", BROKER_HOST)
        .set("group.id", "svix_bridge_test_group_id")
        .create()
        .unwrap();

    consumer.subscribe(&[topic]).unwrap();

    let consumer = Arc::new(consumer);
    let recv_join_hdl = tokio::spawn({
        let consumer = consumer.clone();
        async move { consumer.recv().await.unwrap().detach() }
    });
    tokio::time::sleep(LISTEN_WAIT_TIME).await;

    let payload = json!({ "test": "payload" });
    let payload_s = payload.to_string();

    // Only then actually send a message
    let producer = KafkaProducer::new(
        "test".into(),
        KafkaOutputOpts::Inner {
            bootstrap_brokers: BROKER_HOST.to_owned(),
            topic: topic.to_owned(),
            security_protocol: svix_bridge_plugin_kafka::KafkaSecurityProtocol::Plaintext,
            debug_contexts: None,
        },
    )
    .unwrap();
    producer.handle(ForwardRequest { payload }).await.unwrap();

    // Assert that the message is received
    let msg = recv_join_hdl.await.unwrap();
    assert_eq!(msg.payload(), Some(payload_s.as_bytes()));

    // Assert that no further messages are received in the next second
    tokio::time::timeout(Duration::from_secs(1), consumer.recv())
        .await
        .expect_err("there must be no further messages");

    delete_topic(&admin_client, topic).await;
}
