use rdkafka::{
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use svix_bridge_types::{async_trait, BoxError, ForwardRequest, ReceiverOutput};

use crate::config::KafkaOutputOpts;

/// Forwards webhook payloads to kafka.
pub struct KafkaProducer {
    name: String,
    topic: String,
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(name: String, opts: KafkaOutputOpts) -> Result<Self, KafkaError> {
        let KafkaOutputOpts::Inner { topic, .. } = &opts;
        let topic = topic.clone();
        let producer = opts.create_producer()?;

        Ok(Self {
            name,
            topic,
            producer,
        })
    }
}

#[async_trait]
impl ReceiverOutput for KafkaProducer {
    fn name(&self) -> &str {
        &self.name
    }

    async fn handle(&self, request: ForwardRequest) -> Result<(), BoxError> {
        self.producer
            .send(
                FutureRecord::<(), _>::to(&self.topic)
                    .payload(&serde_json::to_vec(&request.payload)?),
                Timeout::Never,
            )
            .await
            .map_err(|(e, _msg)| e)?;

        Ok(())
    }
}
