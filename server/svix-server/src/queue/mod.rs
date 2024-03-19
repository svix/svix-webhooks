use std::{sync::Arc, time::Duration};

use omniqueue::{
    backends::InMemoryBackend, Delivery, DynConsumer, DynScheduledProducer, QueueConsumer,
    ScheduledQueueProducer,
};
use serde::{Deserialize, Serialize};

use crate::{
    cfg::{Configuration, QueueBackend},
    core::{
        retry::{run_with_retries, Retry},
        types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
    },
    error::{Error, ErrorType, Result, Traceable},
};

pub mod rabbitmq;
pub mod redis;

const RETRY_SCHEDULE: &[Duration] = &[
    Duration::from_millis(10),
    Duration::from_millis(20),
    Duration::from_millis(40),
];

fn should_retry(err: &Error) -> bool {
    matches!(err.typ, ErrorType::Queue(_))
}

pub async fn new_pair(
    cfg: &Configuration,
    prefix: Option<&str>,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    match cfg.queue_backend() {
        QueueBackend::Redis(_) | QueueBackend::RedisCluster(_) => {
            redis::new_pair(cfg, prefix).await
        }
        QueueBackend::Memory => {
            let (producer, consumer) = InMemoryBackend::builder()
                .build_pair()
                .await
                .expect("building in-memory queue can't fail");

            (
                TaskQueueProducer::new(producer),
                TaskQueueConsumer::new(consumer),
            )
        }
        QueueBackend::RabbitMq(dsn) => {
            let prefix = prefix.unwrap_or("");
            let queue = format!("{prefix}-message-queue");
            // Default to a prefetch_size of 1, as it's the safest (least likely to starve consumers)
            let prefetch_size = cfg.rabbit_consumer_prefetch_size.unwrap_or(1);
            rabbitmq::new_pair(dsn, queue, prefetch_size)
                .await
                .expect("can't connect to rabbit")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageTask {
    pub msg_id: MessageId,
    pub app_id: ApplicationId,
    pub endpoint_id: EndpointId,
    pub trigger_type: MessageAttemptTriggerType,
    pub attempt_count: u16,
}

impl MessageTask {
    pub fn new_task(
        msg_id: MessageId,
        app_id: ApplicationId,
        endpoint_id: EndpointId,
        trigger_type: MessageAttemptTriggerType,
    ) -> QueueTask {
        QueueTask::MessageV1(Self {
            msg_id,
            app_id,
            endpoint_id,
            attempt_count: 0,
            trigger_type,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageTaskBatch {
    pub msg_id: MessageId,
    pub app_id: ApplicationId,
    pub force_endpoint: Option<EndpointId>,
    pub trigger_type: MessageAttemptTriggerType,
}

impl MessageTaskBatch {
    pub fn new_task(
        msg_id: MessageId,
        app_id: ApplicationId,
        force_endpoint: Option<EndpointId>,
        trigger_type: MessageAttemptTriggerType,
    ) -> QueueTask {
        QueueTask::MessageBatch(Self {
            msg_id,
            app_id,
            force_endpoint,
            trigger_type,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum QueueTask {
    HealthCheck,
    MessageV1(MessageTask),
    MessageBatch(MessageTaskBatch),
}

impl QueueTask {
    /// Returns a type string, for logging.
    pub fn task_type(&self) -> &'static str {
        match self {
            QueueTask::HealthCheck => "HealthCheck",
            QueueTask::MessageV1(_) => "MessageV1",
            QueueTask::MessageBatch(_) => "MessageBatch",
        }
    }

    pub fn msg_id(&self) -> Option<&str> {
        match self {
            QueueTask::HealthCheck => None,
            QueueTask::MessageV1(v1) => Some(&v1.msg_id),
            QueueTask::MessageBatch(batch) => Some(&batch.msg_id),
        }
    }
}

#[derive(Clone)]
pub struct TaskQueueProducer {
    inner: Arc<DynScheduledProducer>,
}

impl TaskQueueProducer {
    pub fn new(inner: impl ScheduledQueueProducer + 'static) -> Self {
        Self {
            inner: Arc::new(inner.into_dyn_scheduled()),
        }
    }

    pub async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let task = Arc::new(task);
        run_with_retries(
            || async {
                if let Some(delay) = delay {
                    self.inner
                        .send_serde_json_scheduled(task.as_ref(), delay)
                        .await
                } else {
                    self.inner.send_serde_json(task.as_ref()).await
                }
                .map_err(Into::into)
            },
            should_retry,
            RETRY_SCHEDULE,
        )
        .await
    }
}

pub struct TaskQueueConsumer {
    inner: DynConsumer,
}

impl TaskQueueConsumer {
    pub fn new(inner: impl QueueConsumer + 'static) -> Self {
        Self {
            inner: inner.into_dyn(),
        }
    }

    pub async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        const MAX_MESSAGES: usize = 128;
        // FIXME(onelson): need to figure out what deadline/duration to use here
        self.inner
            .receive_all(MAX_MESSAGES, Duration::from_secs(30))
            .await
            .map_err(Into::into)
            .trace()?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}

#[derive(Debug)]
pub struct TaskQueueDelivery {
    pub task: Arc<QueueTask>,
    acker: Delivery,
}

impl TaskQueueDelivery {
    pub async fn ack(self) -> Result<()> {
        tracing::trace!(msg_id = self.task.msg_id(), "ack");

        let mut retry = Retry::new(should_retry, RETRY_SCHEDULE);
        let mut acker = Some(self.acker);
        loop {
            if let Some(result) = retry
                .run(|| async {
                    let delivery = acker
                        .take()
                        .expect("acker is always Some when trying to ack");
                    delivery.ack().await.map_err(|(e, delivery)| {
                        // Put the delivery back in acker before retrying, to
                        // satisfy the expect above.
                        acker = Some(delivery);
                        e.into()
                    })
                })
                .await
            {
                return result;
            }
        }
    }

    pub async fn nack(self) -> Result<()> {
        tracing::trace!(msg_id = self.task.msg_id(), "nack");

        let mut retry = Retry::new(should_retry, RETRY_SCHEDULE);
        let mut acker = Some(self.acker);
        loop {
            if let Some(result) = retry
                .run(|| async {
                    let delivery = acker
                        .take()
                        .expect("acker is always Some when trying to ack");

                    delivery
                        .nack()
                        .await
                        .map_err(|(e, delivery)| {
                            // Put the delivery back in acker before retrying, to
                            // satisfy the expect above.
                            acker = Some(delivery);
                            e.into()
                        })
                        .trace()
                })
                .await
            {
                return result;
            }
        }
    }
}

impl TryFrom<Delivery> for TaskQueueDelivery {
    type Error = Error;
    fn try_from(value: Delivery) -> Result<Self> {
        Ok(TaskQueueDelivery {
            task: Arc::new(
                value
                    .payload_serde_json()
                    .map_err(|_| Error::queue("Failed to decode queue task"))?
                    .ok_or_else(|| Error::queue("Unexpected empty delivery"))?,
            ),
            acker: value,
        })
    }
}
