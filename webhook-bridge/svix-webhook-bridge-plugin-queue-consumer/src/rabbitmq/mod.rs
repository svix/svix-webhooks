use crate::config::{RabbitMqConsumerConfig, RabbitMqInputOpts};
use crate::error::Error;
use crate::run_inner;
use crate::Consumer;
use crate::ConsumerWrapper;
use generic_queue::{
    rabbitmq::{
        BasicProperties, BasicPublishOptions, ConnectionProperties, RabbitMqBackend, RabbitMqConfig,
    },
    TaskQueueBackend,
};
use svix::api::Svix;
use svix_webhook_bridge_types::{async_trait, JsObject, Plugin, TransformerTx};

pub struct RabbitMqConsumerPlugin {
    input_options: RabbitMqInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<String>,
}

impl RabbitMqConsumerPlugin {
    pub fn new(
        RabbitMqConsumerConfig {
            input,
            transformation,
            output,
        }: RabbitMqConsumerConfig,
    ) -> Self {
        Self {
            input_options: input,
            svix_client: Svix::new(output.token, output.svix_options.map(Into::into)),
            transformer_tx: None,
            transformation,
        }
    }
}

impl TryInto<Box<dyn Plugin>> for RabbitMqConsumerConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(RabbitMqConsumerPlugin::new(self)))
    }
}

#[async_trait]
impl Consumer for RabbitMqConsumerPlugin {
    fn source(&self) -> &str {
        &self.input_options.queue_name
    }

    fn system(&self) -> &str {
        "rabbitmq"
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
            <RabbitMqBackend as TaskQueueBackend<JsObject>>::consuming_half(RabbitMqConfig {
                uri: self.input_options.uri.clone(),
                connection_properties: ConnectionProperties::default(),
                publish_exchange: String::new(),
                publish_routing_key: String::new(),
                publish_options: BasicPublishOptions::default(),
                publish_properites: BasicProperties::default(),
                consume_queue: self.input_options.queue_name.clone(),
                consumer_tag: self.input_options.consumer_tag.clone().unwrap_or_default(),
                consume_options: self.input_options.consume_opts.unwrap_or_default(),
                consume_arguments: self.input_options.consume_args.clone().unwrap_or_default(),
                requeue_on_nack: self.input_options.requeue_on_nack,
            })
            .await
            .map_err(Error::from)?;
        Ok(ConsumerWrapper::RabbitMQ(consumer))
    }
}

#[async_trait]
impl Plugin for RabbitMqConsumerPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }
    async fn run(&self) -> std::io::Result<()> {
        run_inner(self).await
    }
}
