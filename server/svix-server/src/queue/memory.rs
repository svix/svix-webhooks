use std::ops::Deref;
use std::{fmt, sync::Arc, time::Duration};

use axum::async_trait;
use omniqueue::backends::memory_queue;
use omniqueue::queue::consumer::QueueConsumer;
use omniqueue::queue::producer::QueueProducer;
use omniqueue::queue::QueueBackend;
use omniqueue::scheduled::ScheduledProducer;

use crate::error::Error;
use crate::error::Result;

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

pub async fn new_pair() -> (TaskQueueProducer, TaskQueueConsumer) {
    let (tx, rx) = memory_queue::MemoryQueueBackend::builder(usize::MAX)
        .build_pair()
        .await
        .expect("memory queue backend");
    (
        TaskQueueProducer::Memory(MemoryQueueProducer { tx: Arc::new(tx) }),
        TaskQueueConsumer::Memory(MemoryQueueConsumer { rx }),
    )
}

#[derive(Clone)]
pub struct MemoryQueueProducer {
    tx: Arc<memory_queue::MemoryQueueProducer>,
}

impl fmt::Debug for MemoryQueueProducer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryQueueProducer").finish()
    }
}

#[async_trait]
impl TaskQueueSend for MemoryQueueProducer {
    async fn send(&self, msg: Arc<QueueTask>, delay: Option<Duration>) -> Result<()> {
        let task = msg.deref().clone();
        if let Some(delay) = delay {
            if self
                .tx
                .send_serde_json_scheduled(&task, delay)
                .await
                .is_err()
            {
                tracing::error!("Receiver dropped");
            }
        } else if self.tx.send_serde_json(&task).await.is_err() {
            tracing::error!("Receiver dropped");
        }

        Ok(())
    }
}

pub struct MemoryQueueConsumer {
    rx: memory_queue::MemoryQueueConsumer,
}

#[async_trait]
impl TaskQueueReceive for MemoryQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        const MAX_MESSAGES: usize = 100;
        let mut tasks: Vec<TaskQueueDelivery> = Vec::with_capacity(MAX_MESSAGES);
        for item in self
            .rx
            .receive_all(MAX_MESSAGES, Duration::from_secs(30))
            .await
            .map_err(|_| Error::queue("Failed to fetch from queue"))?
        {
            tasks.push(item.try_into()?);
        }

        Ok(tasks)
    }
}
