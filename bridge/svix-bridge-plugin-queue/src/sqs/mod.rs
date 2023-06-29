use crate::error::Error;
use crate::{run_inner, Consumer, ConsumerWrapper};
use generic_queue::{
    sqs::{SqsConfig, SqsQueueBackend},
    TaskQueueBackend,
};
use serde::Deserialize;
use svix_bridge_types::{
    async_trait, svix::api::Svix, JsObject, SenderInput, SenderOutputOpts, TransformationConfig,
    TransformerTx,
};

#[derive(Debug, Default, Deserialize)]
pub struct SqsInputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
}

pub struct SqsConsumerPlugin {
    name: String,
    input_options: SqsInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<TransformationConfig>,
}

impl SqsConsumerPlugin {
    pub fn new(
        name: String,
        input: SqsInputOpts,
        transformation: Option<TransformationConfig>,
        output: SenderOutputOpts,
    ) -> Self {
        Self {
            name,
            input_options: input,
            svix_client: match output {
                SenderOutputOpts::Svix(output) => {
                    Svix::new(output.token, output.options.map(Into::into))
                }
            },
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

    fn transformation(&self) -> &Option<TransformationConfig> {
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
impl SenderInput for SqsConsumerPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SqsOutputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
}
