use chrono::Utc;
use futures::StreamExt;
use lapin::{
    options::{
        BasicConsumeOptions, BasicPublishOptions, BasicQosOptions, ExchangeDeclareOptions,
        QueueBindOptions, QueueDeclareOptions,
    },
    types::{AMQPValue, FieldTable},
    BasicProperties, ConnectionProperties,
};
use svix_ksuid::{KsuidLike, KsuidMs};

use crate::{ctx, err_generic, err_queue, error::Result};
use std::{sync::Arc, time::Duration};

use super::{
    Acker, QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

#[derive(Clone)]
pub struct Producer(Arc<ProducerInner>);

struct ProducerInner {
    exchange_name: String,
    queue_name: String,
    channel: lapin::Channel,
}

pub struct Consumer {
    consumer: lapin::Consumer,
}

/// Returns a new_pair producers/consumers that use RabbitMQ under the hood.
/// USE WITH CAUTION - For the time being, this implementation has only been exercised in local development
/// and testing environments. There may be production kinks that need working out
pub async fn new_pair(
    dsn: &str,
    queue_name: String,
    prefetch_size: u16,
) -> Result<(TaskQueueProducer, TaskQueueConsumer)> {
    let conn = ctx!(lapin::Connection::connect(dsn, ConnectionProperties::default()).await)?;
    let producer_chan = ctx!(conn.create_channel().await)?;
    let consumer_chan = ctx!(conn.create_channel().await)?;

    let exchange_name = ctx!(declare_delayed_message_exchange(&producer_chan).await)?;
    ctx!(declare_bound_queue(&queue_name, &exchange_name, &producer_chan).await)?;

    let consumer = ctx!(start_queue_consumer(&queue_name, &consumer_chan, prefetch_size).await)?;
    let consumer = Consumer { consumer };

    let producer = Producer(Arc::new(ProducerInner {
        exchange_name,
        channel: producer_chan,
        queue_name,
    }));

    let producer = TaskQueueProducer::RabbitMq(producer);
    let consumer = TaskQueueConsumer::RabbitMq(consumer);

    Ok((producer, consumer))
}

async fn declare_delayed_message_exchange(channel: &lapin::Channel) -> Result<String> {
    let exchange_name = "first-message";

    // See https://www.rabbitmq.com/amqp-0-9-1-reference.html#exchange.declare
    let opts = ExchangeDeclareOptions {
        // Want the server to create the exchange if it doesn't already exist
        passive: false,
        nowait: false,
        // Want the exchange to survive restarts, and not be deleted even when the underlying queues are
        // deleted
        durable: true,
        auto_delete: false,
        internal: false,
    };

    // See https://github.com/rabbitmq/rabbitmq-delayed-message-exchange#usage
    let mut args = FieldTable::default();
    args.insert(
        "x-delayed-type".into(),
        AMQPValue::LongString("direct".into()),
    );

    ctx!(
        channel
            .exchange_declare(
                exchange_name,
                lapin::ExchangeKind::Custom("x-delayed-message".into()),
                opts,
                args
            )
            .await
    )?;

    Ok(exchange_name.to_owned())
}

async fn declare_bound_queue(
    queue_name: &str,
    exchange_name: &str,
    channel: &lapin::Channel,
) -> Result<()> {
    // Ref https://www.rabbitmq.com/amqp-0-9-1-quickref.html#queue.declare
    let opts = QueueDeclareOptions {
        // If the queue already exists with the same configuration, we want the server to respond w/OK
        // Alternatively, if the queue exists with a _different_ configuration, we want this to fail
        passive: false,
        nowait: false,

        // We want to support multiple consumers per queue
        exclusive: false,

        // We want the queue to survive rust-primary restarts
        durable: true,
        auto_delete: false,
    };

    // Refs https://www.rabbitmq.com/maxlength.html#definition-using-x-args and https://www.rabbitmq.com/dlx.html#using-optional-queue-arguments
    // We may want to figure out what the queue length enforcement looks like and dead letter queueing at a later point in time
    let args = FieldTable::default();
    ctx!(channel.queue_declare(queue_name, opts, args).await)?;

    let routing_key = queue_name;
    ctx!(
        channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                QueueBindOptions { nowait: false },
                FieldTable::default()
            )
            .await
    )?;

    Ok(())
}

async fn start_queue_consumer(
    queue_name: &str,
    channel: &lapin::Channel,
    prefetch_size: u16,
) -> Result<lapin::Consumer> {
    // Ref https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.consume.consumer-tag
    let consumer_tag = format!(
        "{queue_name}-consumer-{}",
        KsuidMs::new(None, None).to_string() // prevent possible errors around duplicate consumer tags
    );

    let opts = BasicConsumeOptions {
        // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-local
        // false because I don't care if the same connection reads and publishes to the same queue
        no_local: false,

        // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-ack
        // Obviously want message ACKs to ensure message are handled
        no_ack: false,

        // https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.consume.exclusive
        // More than one worker should be able to read from the same queue
        exclusive: false,

        // https://www.rabbitmq.com/amqp-0-9-1-reference.html#domain.no-wait
        // want the server to respond if there's a failure
        nowait: false,
    };

    let args = FieldTable::default();

    // Ref https://www.rabbitmq.com/amqp-0-9-1-reference.html#basic.qos.prefetch-size
    //
    // prefetch_size tells the consumer how many messages to load in batches from the queue.
    // Higher values generally means better queue performance (fewer messages stuck in the queue),
    // at the cost of consumer memory.
    // Additionally, if the prefetch_size is *too* large, a single worker can "starve" other workers,
    // potentially hurting total message throughput.
    //
    // "global" enforces the same limit for other consumers on the channel, which isn't necessarily
    // what we want
    ctx!(
        channel
            .basic_qos(prefetch_size, BasicQosOptions { global: false })
            .await
    )?;

    ctx!(
        channel
            .basic_consume(queue_name, &consumer_tag, opts, args)
            .await
    )
}

#[axum::async_trait]
impl TaskQueueSend for Producer {
    async fn send(&self, task: Arc<QueueTask>, delay: Option<Duration>) -> Result<()> {
        let payload = serde_json::to_vec(&task)
            .map_err(|e| err_generic!("unable to serialize queue task wtf: {:?}", e))?;

        let mut headers = FieldTable::default();
        if let Some(delay) = delay {
            let delay_ms: u32 = delay
                .as_millis()
                .try_into()
                .map_err(|_| err_queue!("message delay is too large"))?;
            headers.insert("x-delay".into(), AMQPValue::LongUInt(delay_ms))
        }

        let routing_key = &self.0.queue_name;

        // Ref https://www.rabbitmq.com/publishers.html#unroutable
        let options = BasicPublishOptions {
            mandatory: true, // so we're alerted if the message is unroutable
            immediate: false,
        };

        let id = KsuidMs::new(Some(Utc::now()), None).to_string();

        let properties = BasicProperties::default()
            .with_message_id(id.into())
            .with_headers(headers);

        let confirm = ctx!(
            self.0
                .channel
                .basic_publish(
                    &self.0.exchange_name,
                    routing_key,
                    options,
                    &payload,
                    properties
                )
                .await
        )?;

        ctx!(confirm.await)?;

        Ok(())
    }
}

#[axum::async_trait]
impl TaskQueueReceive for Consumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        // Unfortunately, lapin::Consumer currently has no API for fetching a batch of messages
        // without potentially blocking for each one. So we'll always return a vec! of length 1
        let delivery = ctx!(self.consumer.next().await.ok_or(err_generic!(
            "rabbitmq consumer unexpectedly returned nothing!"
        ))?)?;

        let id = delivery
            .properties
            .message_id()
            .as_ref()
            .ok_or(err_generic!("task is missing message_id!"))?
            .to_string();

        let task: QueueTask = serde_json::from_slice(&delivery.data).map_err(|_e| {
            err_generic!("rabbitmq task deserialization unexpectedly failed?!: {e:?}")
        })?;

        Ok(vec![TaskQueueDelivery {
            id,
            task: Arc::new(task),
            acker: Acker::RabbitMQ(delivery),
        }])
    }
}
