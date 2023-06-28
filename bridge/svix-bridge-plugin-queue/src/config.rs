pub use crate::gcp_pubsub::{GCPPubSubInputOpts, GCPPubSubOutputOpts};
pub use crate::rabbitmq::{RabbitMqInputOpts, RabbitMqOutputOpts};
pub use crate::receiver_output::QueueForwarder;
pub use crate::redis::{RedisInputOpts, RedisOutputOpts};
pub use crate::sqs::{SqsInputOpts, SqsOutputOpts};
use crate::{
    GCPPubSubConsumerPlugin, RabbitMqConsumerPlugin, RedisConsumerPlugin, SqsConsumerPlugin,
};
use serde::Deserialize;
use svix_bridge_types::{
    ReceiverOutput, SenderInput, SenderOutputOpts, TransformationConfig, TransformerInputFormat,
};

#[derive(Deserialize)]
pub struct QueueConsumerConfig {
    pub name: String,
    pub input: SenderInputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: SenderOutputOpts,
}

impl QueueConsumerConfig {
    pub fn into_sender_input(self) -> Result<Box<dyn SenderInput>, &'static str> {
        match self.input {
            SenderInputOpts::GCPPubSub(input) => Ok(Box::new(GCPPubSubConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            ))),
            SenderInputOpts::RabbitMQ(input) => Ok(Box::new(RabbitMqConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            ))),
            SenderInputOpts::Redis(input) => {
                if let Some(xform) = &self.transformation {
                    if xform.format() != TransformerInputFormat::Json {
                        return Err("redis only supports json formatted transformations");
                    }
                }
                Ok(Box::new(RedisConsumerPlugin::new(
                    self.name,
                    input,
                    self.transformation,
                    self.output,
                )))
            }
            SenderInputOpts::SQS(input) => Ok(Box::new(SqsConsumerPlugin::new(
                self.name,
                input,
                self.transformation,
                self.output,
            ))),
        }
    }
}

pub async fn into_receiver_output(
    name: String,
    opts: ReceiverOutputOpts,
    // Annoying to have to pass this, but certain backends (redis) only work with certain transformations (json).
    transformation: &Option<TransformationConfig>,
) -> Result<Box<dyn ReceiverOutput>, crate::Error> {
    let forwarder = match opts {
        ReceiverOutputOpts::GCPPubSub(opts) => {
            QueueForwarder::from_gcp_pupsub_cfg(name, opts).await?
        }
        ReceiverOutputOpts::RabbitMQ(opts) => QueueForwarder::from_rabbitmq_cfg(name, opts).await?,
        ReceiverOutputOpts::Redis(opts) => {
            if let Some(t) = transformation {
                if t.format() != TransformerInputFormat::Json {
                    return Err(crate::Error::Generic(
                        "redis only supports json formatted transformations".to_string(),
                    ));
                }
            }
            QueueForwarder::from_redis_cfg(name, opts).await?
        }
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

#[cfg(test)]
mod tests {
    use super::into_receiver_output;
    use super::QueueConsumerConfig;
    use crate::config::{ReceiverOutputOpts, SenderInputOpts};
    use crate::redis::{RedisInputOpts, RedisOutputOpts};
    use svix_bridge_types::{
        SenderOutputOpts, SvixSenderOutputOpts, TransformationConfig, TransformerInputFormat,
    };

    // FIXME: can't support raw payload access for redis because it requires JSON internally.
    //   Revisit after `omniqueue` adoption.
    #[test]
    fn redis_sender_with_string_transformation_is_err() {
        let cfg = QueueConsumerConfig {
            name: "redis-with-string-transformation".to_string(),
            input: SenderInputOpts::Redis(RedisInputOpts {
                dsn: "".to_string(),
                max_connections: 0,
                reinsert_on_nack: false,
                queue_key: "".to_string(),
                consumer_group: "".to_string(),
                consumer_name: "".to_string(),
            }),
            transformation: Some(TransformationConfig::Explicit {
                format: TransformerInputFormat::String,
                src: String::new(),
            }),
            output: SenderOutputOpts::Svix(SvixSenderOutputOpts {
                token: "".to_string(),
                options: None,
            }),
        };

        assert_eq!(
            cfg.into_sender_input()
                .err()
                .expect("invalid config didn't result in error"),
            "redis only supports json formatted transformations"
        )
    }

    // FIXME: can't support raw payload access for redis because it requires JSON internally.
    //   Revisit after `omniqueue` adoption.
    #[tokio::test]
    async fn test_redis_receiver_string_transform_is_err() {
        let redis_out = ReceiverOutputOpts::Redis(RedisOutputOpts {
            dsn: "".to_string(),
            max_connections: 0,
            queue_key: "".to_string(),
        });

        // Explicit String fails
        let res = into_receiver_output(
            "".to_string(),
            redis_out,
            &Some(TransformationConfig::Explicit {
                src: String::new(),
                format: TransformerInputFormat::String,
            }),
        )
        .await;
        assert!(matches!(
            res.err()
                .expect("invalid config didn't result in error"),
            crate::error::Error::Generic(msg) if msg == "redis only supports json formatted transformations"
        ));
    }
}
