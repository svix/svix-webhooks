use std::time::{Duration, Instant};

use omniqueue::{Delivery, DynConsumer, QueueError};
use svix_bridge_types::{
    async_trait, svix::api::Svix, CreateMessageRequest, JsObject, TransformationConfig,
    TransformerInput, TransformerInputFormat, TransformerJob, TransformerOutput, TransformerTx,
};

pub const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const PLUGIN_VERS: &str = env!("CARGO_PKG_VERSION");

pub mod config;
mod error;
mod gcp_pubsub;
mod rabbitmq;
mod receiver_output;
mod redis;
pub mod sender_input;
mod sqs;

use error::Error;

/// Newtype for [`omniqueue::queue::Delivery`].
///
/// Mostly vestigial at this point, though it doesn't hurt to have something to act as a facade to
/// group helper functions for handling payload details.
pub struct DeliveryWrapper(Delivery);

impl From<Delivery> for DeliveryWrapper {
    fn from(value: Delivery) -> Self {
        Self(value)
    }
}

impl DeliveryWrapper {
    /// Delegates to the inner delivery types ack method.
    async fn ack(self) -> Result<(), QueueError> {
        self.0.ack().await.map_err(|(e, _)| e)
    }
    /// Delegates to the inner delivery types nack method.
    async fn nack(self) -> Result<(), QueueError> {
        self.0.nack().await.map_err(|(e, _)| e)
    }

    /// Decodes the inner delivery as String.
    fn raw_payload(&self) -> Result<&str, QueueError> {
        // TODO: used to be unsupported for redis. Is it now? Check for skipped tests to prove it.
        let bytes = self.0.borrow_payload().ok_or(QueueError::NoData)?;
        std::str::from_utf8(bytes).map_err(QueueError::generic)
    }

    /// Decodes the inner delivery as `serde_json::Value`.
    fn payload(&self) -> Result<serde_json::Value, QueueError> {
        self.0.payload_serde_json()?.ok_or(QueueError::NoData)
    }
}

#[async_trait]
trait Consumer {
    /// The source of the stream of messages, e.g. the name or id for the queue, subscription, etc.
    fn source(&self) -> &str;
    /// The name of the messaging system, e.g. rabbitmq, sqs, etc.
    fn system(&self) -> &str;
    /// Gets the channel sender for running transformations.
    fn transformer_tx(&self) -> Option<&TransformerTx>;
    /// The js source for the transformation to run on each payload.
    fn transformation(&self) -> Option<&TransformationConfig>;
    /// The client to use when creating messages in svix.
    fn svix_client(&self) -> &Svix;

    async fn transform(
        &self,
        script: String,
        input: TransformerInput,
    ) -> std::io::Result<JsObject> {
        let (job, rx) = TransformerJob::new(script, input);
        self.transformer_tx()
            .as_ref()
            .expect("transformations not configured")
            .send(job)
            .map_err(|e| Error::Generic(e.to_string()))?;

        let ret = rx
            .await
            .map_err(|_e| Error::Generic("transformation rx failed".to_string()))
            .and_then(|x| {
                x.map_err(|_e| Error::Generic("transformation execution failed".to_string()))
            })?;

        match ret {
            TransformerOutput::Object(v) => Ok(v),
            TransformerOutput::Invalid => {
                Err(Error::Generic("transformation produced unexpected value".to_string()).into())
            }
        }
    }

    /// Gets consumer (likely based on a config value), called by [`consume`].
    async fn consumer(&self) -> std::io::Result<DynConsumer>;

    /// Main consumer loop
    async fn consume(&self) -> std::io::Result<()> {
        let mut consumer = self.consumer().await?;
        tracing::debug!("{} consuming: {}", self.system(), self.source(),);
        loop {
            self.receive(&mut consumer).await?;
        }
    }

    /// Pulls N messages off the queue and feeds them to [`Self::process`].
    #[tracing::instrument(skip_all,
    fields(
        otel.kind = "CONSUMER",
        messaging.system = self.system(),
        messaging.operation = "receive",
        messaging.source = self.source(),
        svix_bridge_plugin.name = crate::PLUGIN_NAME,
        svix_bridge_plugin.vers = crate::PLUGIN_VERS,
    )
    )]
    async fn receive(&self, consumer: &mut DynConsumer) -> std::io::Result<()> {
        // FIXME: omniqueue has a fixed batch size of 1 afaict. Would be nicer to pull N at a time.
        let delivery = consumer.receive().await.map_err(Error::from)?;
        self.process(delivery.into()).await?;
        Ok(())
    }

    /// Parses the delivery as JSON and feeds it into [`create_svix_message`].
    /// Will nack the delivery if either the JSON parse, transformation, or the request to svix fails.
    #[tracing::instrument(skip_all, fields(messaging.operation = "process"))]
    async fn process(&self, delivery: DeliveryWrapper) -> std::io::Result<()> {
        let payload = if let Some(xform_cfg) = self.transformation() {
            let input = match xform_cfg.format() {
                TransformerInputFormat::Json => {
                    let json_payload = match delivery.payload() {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::warn!("{e}");
                            delivery.nack().await.map_err(Error::from)?;
                            return Ok(());
                        }
                    };
                    TransformerInput::JSON(json_payload)
                }
                TransformerInputFormat::String => {
                    // N.b. our redis backend doesn't support string payloads, but higher up in the
                    // call stack, during the plugin construction, we should be catching this and
                    // giving an error about bad config.
                    // If we get here somehow with a redis delivery, this call will panic.
                    let raw_payload = match delivery.raw_payload() {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::warn!("{e}");
                            delivery.nack().await.map_err(Error::from)?;
                            return Ok(());
                        }
                    };
                    // FIXME: if we add a lifetime to `TransformerInput` we might avoid this allocation.
                    TransformerInput::String(raw_payload.to_string())
                }
            };
            let script = xform_cfg.source().clone();
            match self.transform(script, input).await {
                Err(e) => {
                    tracing::error!("nack: {e}");
                    delivery.nack().await.map_err(Error::from)?;
                    return Ok(());
                }
                Ok(x) => serde_json::from_value(serde_json::Value::Object(x))?,
            }
        } else {
            // Parse as JSON when not using a transformation because Create Message requires JSON.
            // If this fails, the config needs to change.
            let json_payload = match delivery.payload() {
                Ok(p) => p,
                Err(e) => {
                    tracing::warn!("{e}");
                    delivery.nack().await.map_err(Error::from)?;
                    return Ok(());
                }
            };
            serde_json::from_value(json_payload)?
        };

        match create_svix_message(self.svix_client(), payload).await {
            Ok(_) => {
                tracing::trace!("ack");
                delivery.ack().await.map_err(Error::from)?
            }
            Err(e) => {
                tracing::error!("nack: {e}");
                delivery.nack().await.map_err(Error::from)?
            }
        }
        Ok(())
    }
}

async fn run_inner(consumer: &(impl Consumer + Send + Sync)) -> std::io::Result<()> {
    let mut fails: u64 = 0;
    let mut last_fail = Instant::now();
    let system_name = consumer.system();
    let source = consumer.source();

    tracing::info!("{system_name} starting: {source}");

    loop {
        if let Err(e) = consumer.consume().await {
            tracing::error!("{e}");
        }

        tracing::error!("{system_name} disconnected: {source}");

        if last_fail.elapsed() > Duration::from_secs(10) {
            // reset the fail count if we didn't have a hiccup in the past short while.
            tracing::trace!("been a while since last fail, resetting count");
            fails = 0;
        } else {
            fails += 1;
        }

        last_fail = Instant::now();
        tokio::time::sleep(Duration::from_millis((300 * fails).min(3000))).await;
    }
}

#[tracing::instrument(skip_all, level = "error", fields(
    app_id,
    event_type = message.event_type
))]
async fn create_svix_message(
    svix: &Svix,
    CreateMessageRequest {
        app_id,
        message,
        post_options,
    }: CreateMessageRequest,
) -> std::io::Result<()> {
    svix.message()
        .create(app_id, message, post_options.map(Into::into))
        .await
        .map_err(Error::from)?;
    Ok(())
}
