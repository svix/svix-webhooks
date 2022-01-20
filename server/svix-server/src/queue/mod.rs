use std::time::Duration;

use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use svix_ksuid::*;

use crate::{
    core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
    error::Result,
};

pub mod memory;
pub mod redis;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum QueueTask {
    MessageV1(MessageTask),
}

pub struct TaskQueueProducer(Box<dyn TaskQueueSend>);

impl Clone for TaskQueueProducer {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl TaskQueueProducer {
    pub async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        self.0.send(task, delay).await
    }

    pub async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("ack {}", delivery.id);
        self.0.ack(delivery).await
    }

    pub async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("nack {}", delivery.id);
        self.0.nack(delivery).await
    }
}

pub struct TaskQueueConsumer(Box<dyn TaskQueueReceive + Send + Sync>);

impl TaskQueueConsumer {
    pub async fn receive(&mut self) -> Result<TaskQueueDelivery> {
        self.0.receive().await
    }
}

pub struct TaskQueueDelivery {
    pub id: String,
    pub task: QueueTask,
}

impl TaskQueueDelivery {
    /// The `timestamp` is when this message will be delivered at
    fn new(task: QueueTask, timestamp: Option<DateTime<Utc>>) -> Self {
        let ksuid = Ksuid::new(timestamp, None);
        Self {
            id: ksuid.to_string(),
            task,
        }
    }
}

#[async_trait]
trait TaskQueueSend: Sync + Send {
    async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()>;
    fn clone_box(&self) -> Box<dyn TaskQueueSend>;

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()>;
    async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()>;
}

impl Clone for Box<dyn TaskQueueSend> {
    fn clone(&self) -> Box<dyn TaskQueueSend> {
        self.clone_box()
    }
}

#[async_trait]
trait TaskQueueReceive {
    async fn receive(&mut self) -> Result<TaskQueueDelivery>;
}
