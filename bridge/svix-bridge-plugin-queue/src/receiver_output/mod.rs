use std::sync::Arc;

use omniqueue::DynProducer;
use svix_bridge_types::{async_trait, ForwardRequest, ReceiverOutput};

use crate::{config::QueueOutputOpts, error::Result};

#[derive(Clone)]
pub struct QueueForwarder {
    name: String,
    // FIXME: if we retain things like the queue name we can show this in the Debug impl
    // FIXME: raw payloads not yet supported for receivers, but probably should be.
    sender: Arc<DynProducer>,
}

impl QueueForwarder {
    pub async fn from_receiver_output_opts(
        name: String,
        opts: QueueOutputOpts,
    ) -> Result<QueueForwarder> {
        let sender = match opts {
            QueueOutputOpts::GCPPubSub(cfg) => crate::gcp_pubsub::producer(&cfg).await?,
            QueueOutputOpts::RabbitMQ(cfg) => crate::rabbitmq::producer(&cfg).await?,
            QueueOutputOpts::Redis(cfg) => crate::redis::producer(&cfg).await?,
            QueueOutputOpts::SQS(cfg) => crate::sqs::producer(&cfg).await?,
        };
        Ok(QueueForwarder {
            name,
            sender: Arc::new(sender),
        })
    }
}

impl std::fmt::Debug for QueueForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueueForwarder").finish()
    }
}

#[async_trait]
impl ReceiverOutput for QueueForwarder {
    fn name(&self) -> &str {
        &self.name
    }
    async fn handle(&self, request: ForwardRequest) -> std::io::Result<()> {
        Ok(self
            .sender
            .send_serde_json(&request.payload)
            .await
            .map_err(crate::Error::from)?)
    }
}
