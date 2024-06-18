use rdkafka::{
    admin::{AdminClient, NewTopic, TopicReplication},
    client::DefaultClientContext,
    types::RDKafkaErrorCode,
    ClientConfig,
};

/// These tests assume a "vanilla" kafka instance, using the default port, creds, exchange...
const BROKER_HOST: &str = "localhost:9094";

fn kafka_admin_client() -> AdminClient<DefaultClientContext> {
    // create does block I/O, but we don't care in tests
    ClientConfig::new()
        .set("bootstrap.servers", BROKER_HOST)
        .create()
        .unwrap()
}

async fn create_topic(admin_client: &AdminClient<DefaultClientContext>, topic: &str) {
    let new_topic = NewTopic::new(topic, 1, TopicReplication::Fixed(1));
    if let Err(e) = admin_client
        .create_topics(&[new_topic], &Default::default())
        .await
    {
        if e.rdkafka_error_code() != Some(RDKafkaErrorCode::TopicAlreadyExists) {
            panic!("{e}");
        }
    }
}

async fn delete_topic(admin_client: &AdminClient<DefaultClientContext>, topic: &str) {
    admin_client
        .delete_topics(&[topic], &Default::default())
        .await
        .unwrap();
}

macro_rules! unique_topic_name {
    () => {
        &format!(
            "test_{}_{}",
            file!()
                .split('/')
                .next_back()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap(),
            line!()
        )
    };
}

mod kafka_consumer;
mod kafka_producer;
