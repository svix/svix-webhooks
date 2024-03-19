use serde::Deserialize;
use svix_bridge_types::{
    ReceiverOutput, SenderInput, SenderOutputOpts, TransformationConfig, TransformerInputFormat,
};

use crate::sender_input::QueueSender;
pub use crate::{
    gcp_pubsub::{GCPPubSubInputOpts, GCPPubSubOutputOpts},
    rabbitmq::{RabbitMqInputOpts, RabbitMqOutputOpts},
    receiver_output::QueueForwarder,
    redis::{RedisInputOpts, RedisOutputOpts},
    sqs::{SqsInputOpts, SqsOutputOpts},
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
        // FIXME: see if this check is still needed. String transforms worked for the omniqueue redis receiver, I think?
        if matches!(self.input, SenderInputOpts::Redis(_))
            && self
                .transformation
                .as_ref()
                .map(|t| t.format() != TransformerInputFormat::Json)
                .unwrap_or_default()
        {
            return Err("redis only supports json formatted transformations");
        }

        Ok(Box::new(QueueSender::new(
            self.name,
            self.input,
            self.transformation,
            self.output,
        )))
    }
}

pub async fn into_receiver_output(
    name: String,
    opts: ReceiverOutputOpts,
    // Annoying to have to pass this, but certain backends (redis) only work with certain transformations (json).
    transformation: Option<&TransformationConfig>,
) -> Result<Box<dyn ReceiverOutput>, crate::Error> {
    // FIXME: see if this check is still needed. String transforms worked for the omniqueue redis receiver, I think?
    if matches!(opts, ReceiverOutputOpts::Redis(_))
        && transformation
            .as_ref()
            .map(|t| t.format() != TransformerInputFormat::Json)
            .unwrap_or_default()
    {
        return Err(crate::Error::Generic(
            "redis only supports json formatted transformations".to_string(),
        ));
    }

    let forwarder = QueueForwarder::from_receiver_output_opts(name, opts).await?;
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
    use svix_bridge_types::{
        SenderOutputOpts, SvixSenderOutputOpts, TransformationConfig, TransformerInputFormat,
    };

    use super::{into_receiver_output, QueueConsumerConfig};
    use crate::{
        config::{ReceiverOutputOpts, SenderInputOpts},
        redis::{RedisInputOpts, RedisOutputOpts},
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
                delayed_queue_key: None,
                consumer_group: "".to_string(),
                consumer_name: "".to_string(),
                ack_deadline_ms: 2_000,
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
            delayed_queue_key: None,
            ack_deadline_ms: 2_000,
        });

        // Explicit String fails
        let res = into_receiver_output(
            "".to_string(),
            redis_out,
            Some(TransformationConfig::Explicit {
                src: String::new(),
                format: TransformerInputFormat::String,
            })
            .as_ref(),
        )
        .await;
        assert!(matches!(
            res.err()
                .expect("invalid config didn't result in error"),
            crate::error::Error::Generic(msg) if msg == "redis only supports json formatted transformations"
        ));
    }
}
