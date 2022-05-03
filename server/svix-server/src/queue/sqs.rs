use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};
use crate::error::{Error, Result};
use aws_sdk_sqs::model::Message;
use axum::async_trait;
use std::time::Duration;

#[derive(Clone)]
pub struct SQSQueueProducer {
    client: aws_sdk_sqs::Client,
    queue_dsn: String,
}

pub struct SQSQueueConsumer {
    client: aws_sdk_sqs::Client,
    queue_dsn: String,
}

pub async fn new_pair(queue_dsn: &str) -> (TaskQueueProducer, TaskQueueConsumer) {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_sqs::Client::new(&shared_config);
    (
        TaskQueueProducer(Box::new(SQSQueueProducer {
            client: client.clone(),
            queue_dsn: queue_dsn.to_owned(),
        })),
        TaskQueueConsumer(Box::new(SQSQueueConsumer {
            client,
            queue_dsn: queue_dsn.to_owned(),
        })),
    )
}

#[async_trait]
impl TaskQueueSend for SQSQueueProducer {
    async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let timestamp =
            delay.map(|delay| chrono::Utc::now() + chrono::Duration::from_std(delay).unwrap());
        let delivery = TaskQueueDelivery::new(task, timestamp);
        self.client
            .send_message()
            .queue_url(&self.queue_dsn)
            .message_body(serde_json::to_string(&delivery.task).expect("FIXME"))
            .send()
            .await
            .map_err(|e| Error::Queue(e.to_string()))?;
        Ok(())
    }

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("Deleting SQS message using receipt handle {}", &delivery.id);
        self.client
            .delete_message()
            .queue_url(&self.queue_dsn)
            .receipt_handle(delivery.id)
            .send()
            .await
            .map_err(|e| Error::Queue(e.to_string()))?;
        Ok(())
    }

    async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::error!("Failed to process SQS message {}", delivery.id);
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        Box::new(self.clone())
    }
}

fn to_delivery(msg: &Message) -> TaskQueueDelivery {
    let id = msg.receipt_handle.as_ref().unwrap().to_string();
    let task: QueueTask = serde_json::from_str(msg.body.as_ref().unwrap()).unwrap();
    TaskQueueDelivery { id, task }
}

#[async_trait]
impl TaskQueueReceive for SQSQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        let rcv_message_output = self
            .client
            .receive_message()
            .set_wait_time_seconds(Some(20))
            .set_max_number_of_messages(Some(10))
            .queue_url(&self.queue_dsn)
            .send()
            .await
            .map_err(|e| Error::Queue(e.to_string()))?;

        let msgs: Vec<TaskQueueDelivery> = rcv_message_output
            .messages
            .unwrap_or_default()
            .iter()
            .map(to_delivery)
            .collect();

        Ok(msgs)
    }
}
