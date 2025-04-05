use std::sync::Arc;

use omniqueue::{DynProducer, QueueError};
use svix_bridge_types::{async_trait, BoxError, ForwardRequest, ReceiverOutput};
use tokio::sync::Mutex;

use crate::{config::QueueOutputOpts, error::Result};

#[derive(Clone)]
pub struct QueueForwarder {
    name: String,
    opts: QueueOutputOpts,
    // FIXME: raw payloads not yet supported for receivers, but probably should be.
    // FIXME: `RwLock` might be better for throughput, but more likely to drive up complexity in
    //   `QueueForwarder::handle`. A better option might be to handle connection recovery inside
    //    omniqueue.
    sender: Arc<Mutex<Option<DynProducer>>>,
}

impl QueueForwarder {
    async fn build_sender(opts: &QueueOutputOpts) -> Result<DynProducer> {
        Ok(match opts {
            QueueOutputOpts::GcpPubSub(cfg) => crate::gcp_pubsub::producer(cfg).await?,
            QueueOutputOpts::RabbitMQ(cfg) => crate::rabbitmq::producer(cfg).await?,
            QueueOutputOpts::Redis(cfg) => crate::redis::producer(cfg).await?,
            QueueOutputOpts::Sqs(cfg) => crate::sqs::producer(cfg).await?,
        })
    }

    pub async fn from_receiver_output_opts(
        name: String,
        opts: QueueOutputOpts,
    ) -> Result<QueueForwarder> {
        Ok(QueueForwarder {
            name,
            opts,
            sender: Arc::new(Mutex::new(None)),
        })
    }
}

impl std::fmt::Debug for QueueForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: see what fields from `opts` we can expose here, branched by the variant.
        f.debug_struct("QueueForwarder").finish()
    }
}

#[async_trait]
impl ReceiverOutput for QueueForwarder {
    fn name(&self) -> &str {
        &self.name
    }

    async fn handle(&self, request: ForwardRequest) -> Result<(), BoxError> {
        let mut sender = self.sender.lock().await;
        // `QueueForwarder` initializes its sender lazily, but also the sender can be discarded if
        // it gets in a bad state.
        // When None, rebuild the sender.
        if sender.is_none() {
            *sender = Some(Self::build_sender(&self.opts).unwrap()?);
        }

        let res = sender
            .as_ref()
            .expect("non-none sender")
            .send_serde_json(&request.payload)
            .await;

        // Certain clients (like rabbitmq) require intervention when the connection to the remote
        // is interrupted. Check for those failure cases and invalidate the sender so it gets
        // rebuilt next time.
        //
        // `QueueError::Generic` boxes the inner error so either we'd need to have a direct dep on
        // `lapin` so we can downcast to inspect specifics, or otherwise we'd need omniqueue to
        // re-export the error types.
        //
        // In practice multiple error types can show up that all mean the same thing so to start,
        // just recycle for any rabbitmq error.
        if let (Err(QueueError::Generic(_err)), QueueOutputOpts::RabbitMQ(_cfg)) =
            (&res, &self.opts)
        {
            let _ = sender.take();
        }

        res?;

        Ok(())
    }
}
