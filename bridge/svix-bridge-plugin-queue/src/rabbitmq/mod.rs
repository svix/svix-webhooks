use omniqueue::{backends, DynConsumer, DynProducer};
use serde::Deserialize;

use crate::error::{Error, Result};

// wrapper because lapin::BasicConsumeOptions no longer implements Deserialize
mod consume_options {
    use omniqueue::backends;
    use serde::{Deserialize, Deserializer};

    #[derive(Debug, Deserialize, Default)]
    struct BasicConsumeOptions {
        pub no_local: bool,
        pub no_ack: bool,
        pub exclusive: bool,
        pub nowait: bool,
    }

    impl From<BasicConsumeOptions> for backends::rabbitmq::BasicConsumeOptions {
        fn from(value: BasicConsumeOptions) -> Self {
            Self {
                no_local: value.no_local,
                no_ack: value.no_ack,
                exclusive: value.exclusive,
                nowait: value.nowait,
            }
        }
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<backends::rabbitmq::BasicConsumeOptions>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: Option<BasicConsumeOptions> = Deserialize::deserialize(deserializer)?;
        Ok(raw.map(Into::into))
    }
}

// wrapper because lapin::BasicPublishOptions no longer implements Deserialize
mod publish_options {
    use omniqueue::backends;
    use serde::{Deserialize, Deserializer};

    #[derive(Debug, Deserialize, Default)]
    struct BasicPublishOptions {
        pub mandatory: bool,
        pub immediate: bool,
    }

    impl From<BasicPublishOptions> for backends::rabbitmq::BasicPublishOptions {
        fn from(value: BasicPublishOptions) -> Self {
            Self {
                mandatory: value.mandatory,
                immediate: value.immediate,
            }
        }
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<backends::rabbitmq::BasicPublishOptions, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: BasicPublishOptions = Deserialize::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

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
    #[serde(default, with = "crate::rabbitmq::consume_options")]
    pub consume_opts: Option<backends::rabbitmq::BasicConsumeOptions>,
    #[serde(default)]
    pub consume_args: Option<backends::rabbitmq::FieldTable>,
    #[serde(default = "default_requeue")]
    pub requeue_on_nack: bool,
}

fn default_requeue() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RabbitMqOutputOpts {
    /// Connection string for RabbitMQ.
    pub uri: String,
    /// The exchange to publish messages to.
    pub exchange: String,
    /// The routing key to publish messages to.
    pub routing_key: String,
    #[serde(default, with = "crate::rabbitmq::publish_options")]
    pub publish_options: backends::rabbitmq::BasicPublishOptions,
    #[serde(default)]
    pub publish_properties: backends::rabbitmq::BasicProperties,
}

pub async fn consumer(cfg: &RabbitMqInputOpts) -> Result<DynConsumer> {
    backends::rabbitmq::RabbitMqBackend::builder(backends::rabbitmq::RabbitMqConfig {
        uri: cfg.uri.clone(),
        connection_properties: backends::rabbitmq::ConnectionProperties::default(),
        publish_exchange: String::new(),
        publish_routing_key: String::new(),
        publish_options: backends::rabbitmq::BasicPublishOptions::default(),
        publish_properties: backends::rabbitmq::BasicProperties::default(),
        consume_queue: cfg.queue_name.clone(),
        consumer_tag: cfg.consumer_tag.clone().unwrap_or_default(),
        consume_options: cfg.consume_opts.unwrap_or_default(),
        consume_arguments: cfg.consume_args.clone().unwrap_or_default(),
        consume_prefetch_count: None,
        requeue_on_nack: cfg.requeue_on_nack,
    })
    .make_dynamic()
    .build_consumer()
    .await
    .map_err(Error::from)
}
pub async fn producer(cfg: &RabbitMqOutputOpts) -> Result<DynProducer> {
    backends::rabbitmq::RabbitMqBackend::builder(backends::rabbitmq::RabbitMqConfig {
        uri: cfg.uri.clone(),
        // N.b the connection properties type is not serde-friendly. If we want to expose some
        // of these settings we'll probably need to provide our own type and build the real one
        // here from cfg.
        connection_properties: Default::default(),
        publish_exchange: cfg.exchange.clone(),
        publish_routing_key: cfg.routing_key.clone(),
        publish_options: cfg.publish_options,
        publish_properties: cfg.publish_properties.clone(),
        // consumer stuff we don't care about
        consume_queue: "".to_string(),
        consumer_tag: "".to_string(),
        consume_options: Default::default(),
        consume_arguments: Default::default(),
        consume_prefetch_count: None,
        requeue_on_nack: false,
    })
    .make_dynamic()
    .build_producer()
    .await
    .map_err(Error::from)
}
