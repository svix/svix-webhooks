pub use crate::gcp_pubsub::{GCPPubSubInputOpts, GCPPubSubOutputOpts};
pub use crate::rabbitmq::{RabbitMqInputOpts, RabbitMqOutputOpts};
pub use crate::receiver_output::QueueForwarder;
pub use crate::redis::{RedisInputOpts, RedisOutputOpts};
pub use crate::sqs::{SqsInputOpts, SqsOutputOpts};
use crate::{
    GCPPubSubConsumerPlugin, RabbitMqConsumerPlugin, RedisConsumerPlugin, SqsConsumerPlugin,
};
use serde::Deserialize;
use svix_bridge_types::{ReceiverOutput, SenderInput, SenderOutputOpts};

#[derive(Deserialize)]
pub struct QueueConsumerConfig {
    pub name: String,
    pub input: SenderInputOpts,
    #[serde(default)]
    pub transformation: Option<String>,
    pub output: SenderOutputOpts,
}

impl TryInto<Box<dyn SenderInput>> for QueueConsumerConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn SenderInput>, Self::Error> {
        Ok(match self.input {
            SenderInputOpts::GCPPubSub(input) => Box::new(GCPPubSubConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            )),
            SenderInputOpts::RabbitMQ(input) => Box::new(RabbitMqConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            )),
            SenderInputOpts::Redis(input) => Box::new(RedisConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            )),
            SenderInputOpts::SQS(input) => Box::new(SqsConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            )),
        })
    }
}

pub async fn into_receiver_output(
    name: String,
    opts: ReceiverOutputOpts,
) -> Result<Box<dyn ReceiverOutput>, crate::Error> {
    let forwarder = match opts {
        ReceiverOutputOpts::GCPPubSub(opts) => {
            QueueForwarder::from_gcp_pupsub_cfg(name, opts).await?
        }
        ReceiverOutputOpts::RabbitMQ(opts) => QueueForwarder::from_rabbitmq_cfg(name, opts).await?,
        ReceiverOutputOpts::Redis(opts) => QueueForwarder::from_redis_cfg(name, opts).await?,
        ReceiverOutputOpts::SQS(opts) => QueueForwarder::from_sqs_cfg(name, opts).await?,
    };
    Ok(Box::new(forwarder))
}

// TODO: feature flag the variants, thread the features down through to generic-queue
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SenderInputOpts {
    #[serde(rename = "gcp-pubsub")]
    GCPPubSub(GCPPubSubInputOpts),
    RabbitMQ(RabbitMqInputOpts),
    Redis(RedisInputOpts),
    SQS(SqsInputOpts),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ReceiverOutputOpts {
    #[serde(rename = "gcp-pubsub")]
    GCPPubSub(GCPPubSubOutputOpts),
    RabbitMQ(RabbitMqOutputOpts),
    Redis(RedisOutputOpts),
    SQS(SqsOutputOpts),
}
