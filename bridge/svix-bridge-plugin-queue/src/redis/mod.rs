use crate::error::Error;
use crate::run_inner;
use crate::Consumer;
use crate::ConsumerWrapper;
use generic_queue::redis::RedisConfig;
use generic_queue::{redis::RedisQueueBackend, TaskQueueBackend};
use serde::Deserialize;
use svix_bridge_types::{
    async_trait, svix::api::Svix, JsObject, SenderInput, SenderOutputOpts, TransformationConfig,
    TransformerTx,
};

#[derive(Debug, Default, Deserialize)]
pub struct RedisInputOpts {
    pub dsn: String,
    pub max_connections: u16,
    #[serde(default = "default_reinsert_on_nack")]
    pub reinsert_on_nack: bool,
    pub queue_key: String,
    pub consumer_group: String,
    pub consumer_name: String,
}

fn default_reinsert_on_nack() -> bool {
    true
}

pub struct RedisConsumerPlugin {
    name: String,
    input_options: RedisInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<TransformationConfig>,
}

impl RedisConsumerPlugin {
    pub fn new(
        name: String,
        input: RedisInputOpts,
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
impl Consumer for RedisConsumerPlugin {
    fn source(&self) -> &str {
        &self.input_options.queue_key
    }

    fn system(&self) -> &str {
        "redis"
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
        let consumer =
            <RedisQueueBackend as TaskQueueBackend<JsObject>>::consuming_half(RedisConfig {
                dsn: self.input_options.dsn.clone(),
                max_connections: self.input_options.max_connections,
                reinsert_on_nack: self.input_options.reinsert_on_nack,
                queue_key: self.input_options.queue_key.clone(),
                consumer_group: self.input_options.consumer_group.clone(),
                consumer_name: self.input_options.consumer_name.clone(),
            })
            .await
            .map_err(Error::from)?;
        Ok(ConsumerWrapper::Redis(consumer))
    }
}

#[async_trait]
impl SenderInput for RedisConsumerPlugin {
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
pub struct RedisOutputOpts {
    pub dsn: String,
    pub max_connections: u16,
    pub queue_key: String,
}
