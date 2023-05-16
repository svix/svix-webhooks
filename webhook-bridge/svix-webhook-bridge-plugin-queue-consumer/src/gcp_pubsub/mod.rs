use crate::config::{GCPPubSubConsumerConfig, GCPPubSubInputOpts};
use crate::error::Error;
use crate::ConsumerWrapper;
use crate::{run_inner, Consumer};
use generic_queue::gcp_pubsub::{GCPPubSubConfig, GCPPubSubQueueBackend};
use generic_queue::TaskQueueBackend;
use svix::api::Svix;
use svix_webhook_bridge_types::{async_trait, JsObject, Plugin, TransformerTx};

pub struct GCPPubSubConsumerPlugin {
    input_options: GCPPubSubInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<String>,
}

impl GCPPubSubConsumerPlugin {
    pub fn new(
        GCPPubSubConsumerConfig {
            input,
            transformation,
            output,
        }: GCPPubSubConsumerConfig,
    ) -> Self {
        Self {
            input_options: input,
            svix_client: Svix::new(output.token, output.svix_options.map(Into::into)),
            transformer_tx: None,
            transformation,
        }
    }
}

#[async_trait]
impl Consumer for GCPPubSubConsumerPlugin {
    fn source(&self) -> &str {
        &self.input_options.subscription_id
    }

    fn system(&self) -> &str {
        "gcp-pubsub"
    }

    fn transformer_tx(&self) -> &Option<TransformerTx> {
        &self.transformer_tx
    }

    fn transformation(&self) -> &Option<String> {
        &self.transformation
    }

    fn svix_client(&self) -> &Svix {
        &self.svix_client
    }

    async fn consumer(&self) -> std::io::Result<ConsumerWrapper> {
        let consumer = <GCPPubSubQueueBackend as TaskQueueBackend<JsObject>>::consuming_half(
            GCPPubSubConfig {
                subscription_id: self.input_options.subscription_id.clone(),
                credentials_file: self.input_options.credentials_file.clone(),
                // Topics are for producers so we don't care
                topic: String::new(),
            },
        )
        .await
        .map_err(Error::from)?;
        Ok(ConsumerWrapper::GCPPubSub(consumer))
    }
}

impl TryInto<Box<dyn Plugin>> for GCPPubSubConsumerConfig {
    type Error = &'static str;
    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(GCPPubSubConsumerPlugin::new(self)))
    }
}

#[async_trait]
impl Plugin for GCPPubSubConsumerPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self).await
    }
}
