use crate::gcp_pubsub::GCPPubSubOutputOpts;
use crate::rabbitmq::RabbitMqOutputOpts;
use crate::redis::RedisOutputOpts;
use crate::sqs::SqsOutputOpts;
use crate::Error;
use generic_queue::gcp_pubsub::{GCPPubSubConfig, GCPPubSubQueueBackend};
use generic_queue::rabbitmq::{RabbitMqBackend, RabbitMqConfig};
use generic_queue::redis::{RedisConfig, RedisQueueBackend};
use generic_queue::sqs::{SqsConfig, SqsQueueBackend};
use generic_queue::{TaskQueueBackend, TaskQueueSend};
use std::sync::Arc;
use svix_bridge_types::{async_trait, ForwardRequest, ReceiverOutput};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct QueueForwarder {
    name: String,
    // FIXME: if we retain things like the queue name we can show this in the Debug impl
    // FIXME: raw payloads not yet supported for receivers, but probably should be.
    sender: Arc<Box<dyn TaskQueueSend<serde_json::Value>>>,
}

impl QueueForwarder {
    pub async fn from_rabbitmq_cfg(
        name: String,
        cfg: RabbitMqOutputOpts,
    ) -> Result<QueueForwarder> {
        let sender = <RabbitMqBackend as TaskQueueBackend<serde_json::Value>>::producing_half(
            RabbitMqConfig {
                uri: cfg.uri,
                // N.b the connection properties type is not serde-friendly. If we want to expose some
                // of these settings we'll probably need to provide our own type and build the real one
                // here from cfg.
                connection_properties: Default::default(),
                publish_exchange: cfg.exchange,
                publish_routing_key: cfg.routing_key,
                publish_options: cfg.publish_options,
                publish_properites: cfg.publish_properties,
                // consumer stuff we don't care about
                consume_queue: "".to_string(),
                consumer_tag: "".to_string(),
                consume_options: Default::default(),
                consume_arguments: Default::default(),
                requeue_on_nack: false,
            },
        )
        .await?;

        Ok(QueueForwarder {
            name,
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_redis_cfg(name: String, cfg: RedisOutputOpts) -> Result<QueueForwarder> {
        let sender = <RedisQueueBackend as TaskQueueBackend<serde_json::Value>>::producing_half(
            RedisConfig {
                dsn: cfg.dsn,
                max_connections: cfg.max_connections,
                queue_key: cfg.queue_key,
                // consumer stuff we don't really care about
                reinsert_on_nack: false,
                consumer_group: "".to_string(),
                consumer_name: "".to_string(),
            },
        )
        .await?;

        Ok(QueueForwarder {
            name,
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_sqs_cfg(name: String, cfg: SqsOutputOpts) -> Result<QueueForwarder> {
        let sender =
            <SqsQueueBackend as TaskQueueBackend<serde_json::Value>>::producing_half(SqsConfig {
                queue_dsn: cfg.queue_dsn,
                override_endpoint: cfg.override_endpoint,
            })
            .await?;

        Ok(QueueForwarder {
            name,
            sender: Arc::new(Box::new(sender)),
        })
    }

    pub async fn from_gcp_pupsub_cfg(
        name: String,
        cfg: GCPPubSubOutputOpts,
    ) -> Result<QueueForwarder> {
        let sender =
            <GCPPubSubQueueBackend as TaskQueueBackend<serde_json::Value>>::producing_half(
                GCPPubSubConfig {
                    topic: cfg.topic,
                    credentials_file: cfg.credentials_file,
                    // Don't need this. Subscriptions are for consumers only.
                    subscription_id: String::new(),
                },
            )
            .await?;

        Ok(QueueForwarder {
            name,
            sender: Arc::new(Box::new(sender)),
        })
    }
}

impl std::fmt::Debug for QueueForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueueForwarder").finish()
    }
}

#[async_trait]
impl ReceiverOutput for QueueForwarder {
    fn name(&self) -> &str {
        &self.name
    }
    async fn handle(&self, request: ForwardRequest) -> std::io::Result<()> {
        self.sender
            .send(request.payload)
            .await
            .map_err(crate::Error::from)?;
        Ok(())
    }
}
