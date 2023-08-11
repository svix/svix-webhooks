use crate::error::Error;
use crate::run_inner;
use crate::Consumer;
use omniqueue::{
    backends,
    queue::{consumer::DynConsumer, QueueBackend},
};
use serde::Deserialize;
use svix_bridge_types::{
    async_trait, svix::api::Svix, SenderInput, SenderOutputOpts, TransformationConfig,
    TransformerTx,
};

#[derive(Debug, Deserialize)]
pub struct RabbitMqInputOpts {
    /// Connection string for RabbitMQ.
    pub uri: String,
    /// The name of the queue to consume from.
    /// N.b. the queue must be declared before the consumer can connect to it.
    pub queue_name: String,
    /// Identifier for the consumer.
    #[serde(default)]
    pub consumer_tag: Option<String>,
    #[serde(default)]
    pub consume_opts: Option<backends::rabbitmq::BasicConsumeOptions>,
    #[serde(default)]
    pub consume_args: Option<backends::rabbitmq::FieldTable>,
    #[serde(default = "default_requeue")]
    pub requeue_on_nack: bool,
}

fn default_requeue() -> bool {
    true
}

pub struct RabbitMqConsumerPlugin {
    name: String,
    input_options: RabbitMqInputOpts,
    svix_client: Svix,
    transformer_tx: Option<TransformerTx>,
    transformation: Option<TransformationConfig>,
}

impl RabbitMqConsumerPlugin {
    pub fn new(
        name: String,
        input: RabbitMqInputOpts,
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

    fn transformation(&self) -> &Option<TransformationConfig> {
        &self.transformation
    }

    fn svix_client(&self) -> &Svix {
        &self.svix_client
    }

    async fn consumer(&self) -> std::io::Result<DynConsumer> {
        let consumer =
            backends::rabbitmq::RabbitMqBackend::builder(backends::rabbitmq::RabbitMqConfig {
                uri: self.input_options.uri.clone(),
                connection_properties: backends::rabbitmq::ConnectionProperties::default(),
                publish_exchange: String::new(),
                publish_routing_key: String::new(),
                publish_options: backends::rabbitmq::BasicPublishOptions::default(),
                publish_properites: backends::rabbitmq::BasicProperties::default(),
                consume_queue: self.input_options.queue_name.clone(),
                consumer_tag: self.input_options.consumer_tag.clone().unwrap_or_default(),
                consume_options: self.input_options.consume_opts.unwrap_or_default(),
                consume_arguments: self.input_options.consume_args.clone().unwrap_or_default(),
                requeue_on_nack: self.input_options.requeue_on_nack,
            })
            .make_dynamic()
            .build_consumer()
            .await
            .map_err(Error::from)?;
        Ok(consumer)
    }
}

#[async_trait]
impl SenderInput for RabbitMqConsumerPlugin {
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
pub struct RabbitMqOutputOpts {
    /// Connection string for RabbitMQ.
    pub uri: String,
    /// The exchange to publish messages to.
    pub exchange: String,
    /// The routing key to publish messages to.
    pub routing_key: String,
    #[serde(default)]
    pub publish_options: backends::rabbitmq::BasicPublishOptions,
    #[serde(default)]
    pub publish_properties: backends::rabbitmq::BasicProperties,
}
