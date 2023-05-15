use crate::config::{SqsConsumerConfig, SqsInputOpts};
use crate::error::Error;
use crate::{run_inner, Consumer, ConsumerWrapper};
use generic_queue::{
    sqs::{SqsConfig, SqsQueueBackend},
    TaskQueueBackend,
};
use svix::api::Svix;
use svix_webhook_bridge_types::{async_trait, JsObject, Plugin, TransformerTx};

pub struct SqsConsumerPlugin {
    input_options: SqsInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<String>,
}

impl TryInto<Box<dyn Plugin>> for SqsConsumerConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(SqsConsumerPlugin::new(self)))
    }
}

impl SqsConsumerPlugin {
    pub fn new(
        SqsConsumerConfig {
            input,
            transformation,
            output,
        }: SqsConsumerConfig,
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
impl Consumer for SqsConsumerPlugin {
    fn source(&self) -> &str {
        &self.input_options.queue_dsn
    }

    fn system(&self) -> &str {
        "sqs"
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
        let consumer = <SqsQueueBackend as TaskQueueBackend<JsObject>>::consuming_half(SqsConfig {
            queue_dsn: self.input_options.queue_dsn.clone(),
            override_endpoint: self.input_options.override_endpoint,
        })
        .await
        .map_err(Error::from)?;
        Ok(ConsumerWrapper::SQS(consumer))
    }
}

#[async_trait]
impl Plugin for SqsConsumerPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self).await
    }
}
