use crate::config::{RedisConsumerConfig, RedisInputOpts};
use crate::error::Error;
use crate::run_inner;
use crate::Consumer;
use crate::ConsumerWrapper;
use generic_queue::redis::RedisConfig;
use generic_queue::{redis::RedisQueueBackend, TaskQueueBackend};
use svix::api::Svix;
use svix_webhook_bridge_types::{async_trait, JsObject, Plugin, TransformerTx};

pub struct RedisConsumerPlugin {
    input_options: RedisInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<String>,
}

impl RedisConsumerPlugin {
    pub fn new(
        RedisConsumerConfig {
            input,
            transformation,
            output,
        }: RedisConsumerConfig,
    ) -> Self {
        Self {
            input_options: input,
            svix_client: Svix::new(output.token, output.svix_options.map(Into::into)),
            transformer_tx: None,
            transformation,
        }
    }
}

impl TryInto<Box<dyn Plugin>> for RedisConsumerConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(RedisConsumerPlugin::new(self)))
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

    fn transformation(&self) -> &Option<String> {
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
impl Plugin for RedisConsumerPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self).await
    }
}
