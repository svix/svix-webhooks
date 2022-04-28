use axum::async_trait;
use std::time::Duration;
use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};
use crate::error::{Error, Result};
use aws_sdk_sqs::model::Message;

#[derive(Clone)]
pub struct SQSQueueProducer {
    client: aws_sdk_sqs::Client,
}

pub struct SQSQueueConsumer {
    client: aws_sdk_sqs::Client,
}


pub async fn new_pair() -> (TaskQueueProducer, TaskQueueConsumer) {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_sqs::Client::new(&shared_config);
    (
        TaskQueueProducer(Box::new(SQSQueueProducer { client: client.clone() })),
        TaskQueueConsumer(Box::new(SQSQueueConsumer { client })),
    )
}

#[async_trait]
impl TaskQueueSend for SQSQueueProducer {
    async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let timestamp = delay.map(|delay| chrono::Utc::now() + chrono::Duration::from_std(delay).unwrap());
        let delivery = TaskQueueDelivery::new(task, timestamp);
        self.client
            .send_message()
            .queue_url("https://sqs.us-east-1.amazonaws.com/799617403160/test")
            .message_body(serde_json::to_string(&delivery.task).expect("FIXME"))
            .send()
            .await
            .map_err(|e| Error::Queue(e.to_string()))?;
        Ok(())
    }

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        tracing::trace!("Deleting SQS message using receipt handle {}", &delivery.id);
            self.client.delete_message()
                .queue_url("https://sqs.us-east-1.amazonaws.com/799617403160/test")
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
    let task : QueueTask = serde_json::from_str(msg.body.as_ref().unwrap()).unwrap();
    TaskQueueDelivery { id, task }
}

#[async_trait]
impl TaskQueueReceive for SQSQueueConsumer {
    async fn receive(&mut self) -> Result<TaskQueueDelivery> {
        let rcv_message_output = self.client
        .receive_message()
        .queue_url("https://sqs.us-east-1.amazonaws.com/799617403160/test")
        .send()
        .await
        .unwrap();

        let msg = rcv_message_output.messages.ok_or(Error::Queue("No message Received".to_owned()))?;
        // FIXME -- return Timeout variant
        let msg = msg.get(0).unwrap();
        Ok(to_delivery(msg))
    }

}