use std::time::Duration;

use axum::async_trait;
use chrono::Utc;
use tokio::{sync::mpsc, time::sleep};

use crate::error::{Error, Result};

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

pub async fn new_pair() -> (TaskQueueProducer, TaskQueueConsumer) {
    let (tx, rx) = mpsc::unbounded_channel::<TaskQueueDelivery>();
    (
        TaskQueueProducer(Box::new(MemoryQueueProducer { tx })),
        TaskQueueConsumer(Box::new(MemoryQueueConsumer { rx })),
    )
}

#[derive(Clone)]
pub struct MemoryQueueProducer {
    tx: mpsc::UnboundedSender<TaskQueueDelivery>,
}

#[async_trait]
impl TaskQueueSend for MemoryQueueProducer {
    async fn send(&self, msg: QueueTask, delay: Option<Duration>) -> Result<()> {
        let tx = self.tx.clone();
        let timestamp = delay.map(|delay| Utc::now() + chrono::Duration::from_std(delay).unwrap());
        let delivery = TaskQueueDelivery::new(msg, timestamp);
        tokio::spawn(async move {
            // We just assume memory queue always works, so we can defer the error handling
            tracing::trace!("MemoryQueue: event sent > (delay: {:?})", delay);
            if let Some(delay) = delay {
                sleep(delay).await;
            }
            if tx.send(delivery).is_err() {
                tracing::error!("Receiver dropped");
            }
        });
        Ok(())
    }

    async fn ack(&self, _delivery: TaskQueueDelivery) -> Result<()> {
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        Box::new(self.clone())
    }
}

pub struct MemoryQueueConsumer {
    rx: mpsc::UnboundedReceiver<TaskQueueDelivery>,
}

#[async_trait]
impl TaskQueueReceive for MemoryQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        if let Some(delivery) = self.rx.recv().await {
            tracing::trace!("MemoryQueue: event recv <");
            Ok(vec![delivery])
        } else {
            Err(Error::Queue("Failed to fetch from queue".to_owned()))
        }
    }
}
