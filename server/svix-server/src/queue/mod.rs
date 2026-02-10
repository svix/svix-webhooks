use std::{marker::PhantomData, num::NonZeroUsize, sync::Arc, time::Duration};

use omniqueue::{
    Delivery, DynConsumer, QueueConsumer, ScheduledQueueProducer, backends::InMemoryBackend,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    cfg::{Configuration, QueueBackend},
    core::{
        retry::{Retry, run_with_retries},
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

pub type TaskQueueDelivery = SvixOmniDelivery<QueueTask>;

fn should_retry(err: &Error) -> bool {
    matches!(err.typ, ErrorType::Queue(_))
}

pub async fn new_pair(
    cfg: &Configuration,
    prefix: Option<&str>,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    match cfg.queue_backend() {
        QueueBackend::Redis(_)
        | QueueBackend::RedisCluster(_)
        | QueueBackend::RedisSentinel(_, _) => redis::new_pair(cfg, prefix).await,
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

pub type TaskQueueProducer = SvixOmniProducer<QueueTask>;
pub type TaskQueueConsumer = SvixOmniConsumer<QueueTask>;

pub struct SvixOmniProducer<T: OmniMessage> {
    inner: Arc<omniqueue::DynScheduledProducer>,
    _phantom: PhantomData<T>,
}

// Manual impl to avoid adding 'Clone' bound on T
impl<T: OmniMessage> Clone for SvixOmniProducer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: OmniMessage> SvixOmniProducer<T> {
    pub(super) fn new(inner: impl ScheduledQueueProducer + 'static) -> Self {
        Self {
            inner: Arc::new(inner.into_dyn_scheduled()),
            _phantom: PhantomData,
        }
    }

    #[tracing::instrument(skip_all, name = "queue_send")]
    pub async fn send(&self, task: &T, delay: Option<Duration>) -> Result<()> {
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

    #[tracing::instrument(skip_all, name = "redrive_dlq")]
    pub async fn redrive_dlq(&self) -> Result<()> {
        self.inner.redrive_dlq().await.map_err(Into::into)
    }
}

pub struct SvixOmniConsumer<T: OmniMessage> {
    inner: DynConsumer,
    _phantom: PhantomData<T>,
}

pub trait OmniMessage: Serialize + DeserializeOwned + Send + Sync {
    fn task_id(&self) -> Option<&str>;
}

impl OmniMessage for QueueTask {
    fn task_id(&self) -> Option<&str> {
        self.msg_id()
    }
}

impl<T: OmniMessage> SvixOmniConsumer<T> {
    pub(super) fn new(inner: impl QueueConsumer + 'static) -> Self {
        Self {
            inner: inner.into_dyn(),
            _phantom: PhantomData,
        }
    }

    #[tracing::instrument(skip_all, name = "queue_receive_all")]
    pub async fn receive_all(&mut self, deadline: Duration) -> Result<Vec<SvixOmniDelivery<T>>> {
        pub const MAX_MESSAGES: usize = 128;
        self.inner
            .receive_all(MAX_MESSAGES, deadline)
            .await
            .map_err(Error::from)
            .trace()?
            .into_iter()
            .map(|acker| {
                Ok(SvixOmniDelivery {
                    task: Arc::new(
                        acker
                            .payload_serde_json()
                            .map_err(|e| {
                                Error::queue(format_args!("Failed to decode queue task: {e:?}"))
                            })?
                            .ok_or_else(|| Error::queue("Unexpected empty delivery"))?,
                    ),

                    acker,
                })
            })
            .collect()
    }

    pub fn max_messages(&self) -> Option<NonZeroUsize> {
        self.inner.max_messages()
    }
}

#[derive(Debug)]
pub struct SvixOmniDelivery<T> {
    pub task: Arc<T>,
    pub(super) acker: Delivery,
}

impl<T: OmniMessage> SvixOmniDelivery<T> {
    pub async fn set_ack_deadline(&mut self, duration: Duration) -> Result<()> {
        Ok(self.acker.set_ack_deadline(duration).await?)
    }
    pub async fn ack(self) -> Result<()> {
        tracing::trace!(
            task_id = self.task.task_id().map(tracing::field::display),
            "ack"
        );

        let mut retry = Retry::new(should_retry, RETRY_SCHEDULE);
        let mut acker = Some(self.acker);
        loop {
            if let Some(result) = retry
                .run(|| async {
                    match acker.take() {
                        Some(delivery) => {
                            delivery.ack().await.map_err(|(e, delivery)| {
                                // Put the delivery back in acker before retrying, to
                                // satisfy the expect above.
                                acker = Some(delivery);
                                e.into()
                            })
                        }
                        None => unreachable!(),
                    }
                })
                .await
            {
                return result;
            }
        }
    }

    pub async fn nack(self) -> Result<()> {
        tracing::trace!(
            task_id = self.task.task_id().map(tracing::field::display),
            "nack"
        );

        let mut retry = Retry::new(should_retry, RETRY_SCHEDULE);
        let mut acker = Some(self.acker);
        loop {
            if let Some(result) = retry
                .run(|| async {
                    match acker.take() {
                        Some(delivery) => {
                            delivery
                                .nack()
                                .await
                                .map_err(|(e, delivery)| {
                                    // Put the delivery back in acker before retrying, to
                                    // satisfy the expect above.
                                    acker = Some(delivery);
                                    Error::from(e)
                                })
                                .trace()
                        }
                        _ => unreachable!(),
                    }
                })
                .await
            {
                return result;
            }
        }
    }
}
