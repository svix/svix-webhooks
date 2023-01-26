use std::{sync::Arc, time::Duration};

use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use svix_ksuid::*;

use crate::{
    cfg::{Configuration, QueueBackend},
    core::{
        run_with_retries::run_with_retries,
        types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
    },
    error::{Error, ErrorType, Result},
};

pub mod memory;
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
        QueueBackend::Redis(dsn) => {
            let pool = crate::redis::new_redis_pool(dsn, cfg).await;
            redis::new_pair(pool, prefix).await
        }
        QueueBackend::RedisCluster(dsn) => {
            let pool = crate::redis::new_redis_pool_clustered(dsn, cfg).await;
            redis::new_pair(pool, prefix).await
        }
        QueueBackend::Memory => memory::new_pair().await,
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
    pub trigger_type: MessageAttemptTriggerType,
}

impl MessageTaskBatch {
    pub fn new_task(
        msg_id: MessageId,
        app_id: ApplicationId,
        trigger_type: MessageAttemptTriggerType,
    ) -> QueueTask {
        QueueTask::MessageBatch(Self {
            msg_id,
            app_id,
            trigger_type,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum QueueTask {
    HealthCheck,
    MessageV1(MessageTask),
    MessageBatch(MessageTaskBatch),
}

pub struct TaskQueueProducer(Box<dyn TaskQueueSend>);

impl Clone for TaskQueueProducer {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl TaskQueueProducer {
    pub async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let task = Arc::new(task);
        run_with_retries(
            || async { self.0.send(task.clone(), delay).await },
            should_retry,
            RETRY_SCHEDULE,
        )
        .await
    }

    pub async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("ack {}", delivery.id);
        run_with_retries(
            || async { self.0.ack(&delivery).await },
            should_retry,
            RETRY_SCHEDULE,
        )
        .await
    }

    pub async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("nack {}", delivery.id);
        run_with_retries(
            || async { self.0.nack(&delivery).await },
            should_retry,
            RETRY_SCHEDULE,
        )
        .await
    }
}

pub struct TaskQueueConsumer(Box<dyn TaskQueueReceive + Send + Sync>);

impl TaskQueueConsumer {
    pub async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        self.0.receive_all().await
    }
}

pub struct TaskQueueDelivery {
    pub id: String,
    pub task: Arc<QueueTask>,
}

impl TaskQueueDelivery {
    /// The `timestamp` is when this message will be delivered at
    fn from_arc(task: Arc<QueueTask>, timestamp: Option<DateTime<Utc>>) -> Self {
        let ksuid = KsuidMs::new(timestamp, None);
        Self {
            id: ksuid.to_string(),
            task,
        }
    }
}

#[async_trait]
trait TaskQueueSend: Sync + Send {
    async fn send(&self, task: Arc<QueueTask>, delay: Option<Duration>) -> Result<()>;
    fn clone_box(&self) -> Box<dyn TaskQueueSend>;

    async fn ack(&self, delivery: &TaskQueueDelivery) -> Result<()>;

    /// By default NACKing a [`TaskQueueDelivery`] simply reinserts it in the back of the queue
    /// without any delay. Additionally it `ack`s the orignal, now duplicated task, such as to
    /// avoid memory leaks in persistent implementations of the queue.
    async fn nack(&self, delivery: &TaskQueueDelivery) -> Result<()> {
        tracing::debug!("nack {}", delivery.id);
        self.send(delivery.task.clone(), None).await?;
        self.ack(delivery).await
    }
}

impl Clone for Box<dyn TaskQueueSend> {
    fn clone(&self) -> Box<dyn TaskQueueSend> {
        self.clone_box()
    }
}

#[async_trait]
trait TaskQueueReceive {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test Redis impl too

    /// Creates a [`MessageTask`] with filler information and the given MessageId inner String
    fn mock_message(message_id: String) -> QueueTask {
        MessageTask::new_task(
            MessageId(message_id),
            ApplicationId("TestEndpointID".to_owned()),
            EndpointId("TestEndpointID".to_owned()),
            MessageAttemptTriggerType::Scheduled,
        )
    }

    /// Sends a message with the given TaskQueueProducer reference and asserts that the result is OK
    async fn assert_send(tx: &TaskQueueProducer, message_id: &str) {
        assert!(tx
            .send(mock_message(message_id.to_owned()), None)
            .await
            .is_ok());
    }

    /// Receives a message with the given TaskQueueConsumer mutable reference and asserts that it is
    /// equal to the mock message with the given message_id.
    async fn assert_recv(rx: &mut TaskQueueConsumer, message_id: &str) {
        assert_eq!(
            *rx.receive_all().await.unwrap().get(0).unwrap().task,
            mock_message(message_id.to_owned())
        )
    }

    #[tokio::test]
    async fn test_single_producer_single_consumer() {
        let (tx_mem, mut rx_mem) = memory::new_pair().await;

        let msg_id = "TestMessageID1";

        assert_send(&tx_mem, msg_id).await;
        assert_recv(&mut rx_mem, msg_id).await;
    }

    #[tokio::test]
    async fn test_multiple_producer_single_consumer() {
        let (tx_mem, mut rx_mem) = memory::new_pair().await;

        let msg_1 = "TestMessageID1";
        let msg_2 = "TestMessageID2";

        tokio::spawn({
            let tx_mem = tx_mem.clone();
            async move {
                assert_send(&tx_mem, msg_1).await;
            }
        });
        tokio::spawn(async move {
            assert_send(&tx_mem, msg_2).await;
        });

        assert_recv(&mut rx_mem, msg_1).await;
        assert_recv(&mut rx_mem, msg_2).await;
    }

    #[tokio::test]
    async fn test_delay() {
        let (tx_mem, mut rx_mem) = memory::new_pair().await;

        let msg_1 = "TestMessageID1";
        let msg_2 = "TestMessageID2";

        assert!(tx_mem
            .send(
                mock_message(msg_1.to_owned()),
                Some(Duration::from_millis(200))
            )
            .await
            .is_ok());
        assert_send(&tx_mem, msg_2).await;

        assert_recv(&mut rx_mem, msg_2).await;
        assert_recv(&mut rx_mem, msg_1).await;
    }

    #[tokio::test]
    async fn test_ack() {
        let (tx_mem, mut rx_mem) = memory::new_pair().await;
        assert!(tx_mem
            .send(mock_message("test".to_owned()), None)
            .await
            .is_ok());

        let recv = rx_mem
            .receive_all()
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        assert_eq!(*recv.task, mock_message("test".to_owned()));

        assert!(tx_mem.ack(recv).await.is_ok());

        tokio::select! {
            _ = rx_mem.receive_all() => {
                panic!("`rx_mem` received second message");
            }

            // FIXME: Find out correct timeout duration
            _ = tokio::time::sleep(Duration::from_millis(500)) => {}
        }
    }

    #[tokio::test]
    async fn test_nack() {
        let (tx_mem, mut rx_mem) = memory::new_pair().await;
        assert!(tx_mem
            .send(mock_message("test".to_owned()), None)
            .await
            .is_ok());

        let recv = rx_mem
            .receive_all()
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert_eq!(*recv.task, mock_message("test".to_owned()));

        assert!(tx_mem.nack(recv).await.is_ok());

        tokio::select! {
            _ = rx_mem.receive_all() => {}

            // FIXME: Find out correct timeout duration
            _ = tokio::time::sleep(Duration::from_millis(500)) => {
                panic!("`rx_mem` did not receive second message");
            }
        }
    }
}
