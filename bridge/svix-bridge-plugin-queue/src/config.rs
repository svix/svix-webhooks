use svix_bridge_types::{
    ReceiverOutput, SenderInput, SenderOutputOpts, TransformationConfig, TransformerInputFormat,
};

use crate::sender_input::QueueSender;
pub use crate::{
    gcp_pubsub::{GcpPubSubInputOpts, GcpPubSubOutputOpts},
    rabbitmq::{RabbitMqInputOpts, RabbitMqOutputOpts},
    receiver_output::QueueForwarder,
    redis::{RedisInputOpts, RedisOutputOpts},
    sqs::{SqsInputOpts, SqsOutputOpts},
};

pub fn into_sender_input(
    name: String,
    input_opts: impl Into<QueueInputOpts>,
    transformation: Option<TransformationConfig>,
    output: SenderOutputOpts,
) -> Result<Box<dyn SenderInput>, &'static str> {
    let input_opts = input_opts.into();
    // FIXME: see if this check is still needed. String transforms worked for the omniqueue redis receiver, I think?
    if matches!(input_opts, QueueInputOpts::Redis(_))
        && transformation
            .as_ref()
            .map(|t| t.format() != TransformerInputFormat::Json)
            .unwrap_or_default()
    {
        return Err("redis only supports json formatted transformations");
    }

    Ok(Box::new(QueueSender::new(
        name,
        input_opts,
        transformation,
        output,
    )))
}

pub async fn into_receiver_output(
    name: String,
    opts: impl Into<QueueOutputOpts>,
    // Annoying to have to pass this, but certain backends (redis) only work with certain transformations (json).
    transformation: Option<&TransformationConfig>,
) -> Result<Box<dyn ReceiverOutput>, crate::Error> {
    let opts = opts.into();
    // FIXME: see if this check is still needed. String transforms worked for the omniqueue redis receiver, I think?
    if matches!(opts, QueueOutputOpts::Redis(_))
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
#[derive(Clone, Debug)]
pub enum QueueInputOpts {
    GcpPubSub(GcpPubSubInputOpts),
    RabbitMQ(RabbitMqInputOpts),
    Redis(RedisInputOpts),
    Sqs(SqsInputOpts),
}

impl From<GcpPubSubInputOpts> for QueueInputOpts {
    fn from(value: GcpPubSubInputOpts) -> Self {
        Self::GcpPubSub(value)
    }
}

impl From<RabbitMqInputOpts> for QueueInputOpts {
    fn from(value: RabbitMqInputOpts) -> Self {
        Self::RabbitMQ(value)
    }
}

impl From<RedisInputOpts> for QueueInputOpts {
    fn from(value: RedisInputOpts) -> Self {
        Self::Redis(value)
    }
}

impl From<SqsInputOpts> for QueueInputOpts {
    fn from(value: SqsInputOpts) -> Self {
        Self::Sqs(value)
    }
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // Largest variant isn't _that_ big
pub enum QueueOutputOpts {
    GcpPubSub(GcpPubSubOutputOpts),
    RabbitMQ(RabbitMqOutputOpts),
    Redis(RedisOutputOpts),
    Sqs(SqsOutputOpts),
}

impl From<GcpPubSubOutputOpts> for QueueOutputOpts {
    fn from(value: GcpPubSubOutputOpts) -> Self {
        Self::GcpPubSub(value)
    }
}

impl From<RabbitMqOutputOpts> for QueueOutputOpts {
    fn from(value: RabbitMqOutputOpts) -> Self {
        Self::RabbitMQ(value)
    }
}

impl From<RedisOutputOpts> for QueueOutputOpts {
    fn from(value: RedisOutputOpts) -> Self {
        Self::Redis(value)
    }
}

impl From<SqsOutputOpts> for QueueOutputOpts {
    fn from(value: SqsOutputOpts) -> Self {
        Self::Sqs(value)
    }
}

#[cfg(test)]
mod tests {
    use svix_bridge_types::{
        SenderOutputOpts, SvixSenderOutputOpts, TransformationConfig, TransformerInputFormat,
    };

    use super::{into_receiver_output, into_sender_input};
    use crate::{
        config::{QueueInputOpts, QueueOutputOpts},
        rabbitmq::RabbitMqInputOpts,
        redis::{RedisInputOpts, RedisOutputOpts},
    };

    // FIXME: can't support raw payload access for redis because it requires JSON internally.
    //   Revisit after `omniqueue` adoption.
    #[test]
    fn redis_sender_with_string_transformation_is_err() {
        let input_opts = QueueInputOpts::Redis(RedisInputOpts {
            dsn: "".to_string(),
            max_connections: 0,
            reinsert_on_nack: false,
            queue_key: "".to_string(),
            delayed_queue_key: None,
            consumer_group: "".to_string(),
            consumer_name: "".to_string(),
            payload_key: "payload".to_string(),
            ack_deadline_ms: 2_000,
            dlq_config: None,
        });

        let err = into_sender_input(
            "redis-with-string-transformation".to_owned(),
            input_opts,
            Some(TransformationConfig::Explicit {
                format: TransformerInputFormat::String,
                src: String::new(),
            }),
            SenderOutputOpts::Svix(SvixSenderOutputOpts {
                token: "".to_string(),
                options: None,
            }),
        )
        .err()
        .expect("invalid config didn't result in error");

        assert_eq!(err, "redis only supports json formatted transformations")
    }

    // FIXME: can't support raw payload access for redis because it requires JSON internally.
    //   Revisit after `omniqueue` adoption.
    #[tokio::test]
    async fn test_redis_receiver_string_transform_is_err() {
        let redis_out = QueueOutputOpts::Redis(RedisOutputOpts {
            dsn: "".to_string(),
            max_connections: 0,
            queue_key: "".to_string(),
            delayed_queue_key: None,
            payload_key: "payload".to_string(),
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

    #[test]
    fn rabbitmq_input_opts_deserializes_consume_prefetch_count() {
        let opts: RabbitMqInputOpts = serde_json::from_value(serde_json::json!({
            "uri": "amqp://guest:guest@localhost:5672/%2f",
            "queue_name": "my-queue",
            "consume_prefetch_count": 10,
        }))
        .unwrap();
        assert_eq!(opts.consume_prefetch_count, Some(10));

        // Absent is fine too, and doesn't default to some arbitrary limit.
        let opts: RabbitMqInputOpts = serde_json::from_value(serde_json::json!({
            "uri": "amqp://guest:guest@localhost:5672/%2f",
            "queue_name": "my-queue",
        }))
        .unwrap();
        assert_eq!(opts.consume_prefetch_count, None);
    }

    #[test]
    fn redis_input_opts_deserializes_dlq_config() {
        let opts: RedisInputOpts = serde_json::from_value(serde_json::json!({
            "dsn": "redis://localhost:6379/",
            "max_connections": 4,
            "queue_key": "my_queue",
            "consumer_group": "my_group",
            "consumer_name": "my_consumer",
            "dlq_config": {
                "queue_key": "my_queue_dlq",
                "max_receives": 5,
            },
        }))
        .unwrap();
        let dlq = opts.dlq_config.expect("dlq_config should be set");
        assert_eq!(dlq.queue_key, "my_queue_dlq");
        assert_eq!(dlq.max_receives, 5);

        // Absent is fine too -- no DLQ by default.
        let opts: RedisInputOpts = serde_json::from_value(serde_json::json!({
            "dsn": "redis://localhost:6379/",
            "max_connections": 4,
            "queue_key": "my_queue",
            "consumer_group": "my_group",
            "consumer_name": "my_consumer",
        }))
        .unwrap();
        assert!(opts.dlq_config.is_none());
    }

    #[test]
    fn redis_opts_deserialize_payload_key() {
        let opts: RedisInputOpts = serde_json::from_value(serde_json::json!({
            "dsn": "redis://localhost:6379/",
            "max_connections": 4,
            "queue_key": "my_queue",
            "consumer_group": "my_group",
            "consumer_name": "my_consumer",
            "payload_key": "custom_payload",
        }))
        .unwrap();
        assert_eq!(opts.payload_key, "custom_payload");

        // Defaults to "payload" when absent.
        let opts: RedisInputOpts = serde_json::from_value(serde_json::json!({
            "dsn": "redis://localhost:6379/",
            "max_connections": 4,
            "queue_key": "my_queue",
            "consumer_group": "my_group",
            "consumer_name": "my_consumer",
        }))
        .unwrap();
        assert_eq!(opts.payload_key, "payload");
    }
}
