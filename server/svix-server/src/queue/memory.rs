use std::{sync::Arc, time::Duration};

use axum::async_trait;
use chrono::Utc;
use tokio::{sync::mpsc, time::sleep};

use crate::{err_queue, error::Result};

use super::{
    Acker, QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

pub async fn new_pair() -> (TaskQueueProducer, TaskQueueConsumer) {
    let (tx, rx) = mpsc::unbounded_channel::<TaskQueueDelivery>();
    (
        TaskQueueProducer::Memory(MemoryQueueProducer { tx }),
        TaskQueueConsumer::Memory(MemoryQueueConsumer { rx }),
    )
}

#[derive(Clone, Debug)]
pub struct MemoryQueueProducer {
    tx: mpsc::UnboundedSender<TaskQueueDelivery>,
}

#[async_trait]
impl TaskQueueSend for MemoryQueueProducer {
    async fn send(&self, msg: Arc<QueueTask>, delay: Option<Duration>) -> Result<()> {
        let timestamp = delay.map(|delay| Utc::now() + chrono::Duration::from_std(delay).unwrap());
        let delivery = TaskQueueDelivery::from_arc(msg, timestamp, Acker::Memory(self.clone()));

        if let Some(delay) = delay {
            let tx = self.tx.clone();
            tokio::spawn(async move {
                // We just assume memory queue always works, so we can defer the error handling
                tracing::trace!("MemoryQueue: event sent > (delay: {:?})", delay);
                sleep(delay).await;
                if tx.send(delivery).is_err() {
                    tracing::error!("Receiver dropped");
                }
            });
        } else if self.tx.send(delivery).is_err() {
            tracing::error!("Receiver dropped");
        }

        Ok(())
    }
}

pub struct MemoryQueueConsumer {
    rx: mpsc::UnboundedReceiver<TaskQueueDelivery>,
}

#[async_trait]
impl TaskQueueReceive for MemoryQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        let mut deliveries = tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(30)) => return Ok(Vec::new()),
            recv = self.rx.recv() => {
                if let Some(delivery) = recv {
                    tracing::trace!("MemoryQueue: event recv <");
                    vec![delivery]
                } else {
                    return Err(err_queue!("Failed to fetch from queue"))
                }
            }
        };

        // possible errors are `Empty` or `Disconnected`. Either way,
        // we want to return the deliveries that could be received.
        // If it was Disconnected, the next call to receive_all will fail
        while let Ok(delivery) = self.rx.try_recv() {
            deliveries.push(delivery);
        }

        Ok(deliveries)
    }
}
