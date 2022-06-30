use axum::async_trait;
use sea_orm::DatabaseConnection;
use std::time::Duration;

use super::{TaskQueueConsumer, TaskQueueProducer, TaskQueueReceive, TaskQueueSend};

const DEFAULT_MAIN: &str = "{queue}_svix_v3_main";
const DEFAULT_DELAYED: &str = "{queue}_svix_delayed";
const DEFAULT_PREFIX: &str = "default";

#[derive(Debug, Clone)]
pub struct PgQueueOpts<'a> {
    pub pool: DatabaseConnection,
    pub main_queue_name: Option<&'a str>,
    pub delayed_queue_name: Option<&'a str>,
    pub prefix: Option<&'a str>,
}

impl PgQueueOpts<'_> {
    pub fn prefix(&self) -> String {
        match self.prefix {
            Some(prefix) => prefix.to_owned(),
            None => DEFAULT_PREFIX.to_owned(),
        }
    }

    pub fn main_queue_name(&self) -> String {
        match self.main_queue_name {
            Some(mqn) => mqn.to_owned(),
            None => DEFAULT_MAIN.to_owned(),
        }
    }
}

/**
 * Whenever we create new (prod, sub) pair,
 * - publisher should be clonable(multi-producer)
 * - we need to make sure before returning this pair, we need to make sure, we start the consumer
 *   and consumer is ready to listen for new events/messages
 * - Ideally we should not be returning the consumer, as for any queue in ideal situation,
 *   there should be one consumer for better consistancy
 */
pub async fn new_pair(opts: PgQueueOpts<'_>) -> (TaskQueueProducer, TaskQueueConsumer) {
    let prefix = opts.prefix();
    let main_queue_name = format!("{}{}", prefix, opts.main_queue_name());

    let task_producer = TaskQueueProducer(Box::new(PgQueueProducer {
        pool: opts.pool.clone(),
        queue_name: opts.main_queue_name().clone(),
    }));

    let pgq_consumer = PgQueueConsumer {
        pool: opts.pool.clone(),
        queue_name: opts.main_queue_name().clone(),
    };

    let task_consumer = TaskQueueConsumer(Box::new(pgq_consumer.clone()));

    PgConsumerListener::new(pgq_consumer.clone()).listen();

    (task_producer, task_consumer)
}

#[derive(Debug, Clone)]
pub struct PgQueueProducer {
    pool: DatabaseConnection,
    queue_name: String,
}

#[async_trait]
impl TaskQueueSend for PgQueueProducer {
    //NOTE: insert the task into backend, in this case, postgresql
    async fn send(
        &self,
        task: super::QueueTask,
        delay: Option<std::time::Duration>,
    ) -> crate::error::Result<()> {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        todo!()
    }

    //NOTE: remove processed task from the backend queue / mark the task as completed
    async fn ack(&self, delivery: super::TaskQueueDelivery) -> crate::error::Result<()> {
        todo!()
    }

    //NOTE: don't do anything as of now, we should be able to re-process this task.
    //NOTE: mostly, can be used for retry mech.
    async fn nack(&self, delivery: super::TaskQueueDelivery) -> crate::error::Result<()> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct PgQueueConsumer {
    pool: DatabaseConnection,
    queue_name: String,
}

#[async_trait]
impl TaskQueueReceive for PgQueueConsumer {
    //NOTE: fetch all the tasks from backend queue for given queue_name.
    //NOTE: as postgres doesn't have anyway to stream tasks, we might have to do check at certain interval
    //NOTE: if not, fetching tasks at certail intervals, we can also implement event mech. not implementing right now
    async fn receive_all(&mut self) -> crate::error::Result<Vec<super::TaskQueueDelivery>> {
        todo!()
    }
}

struct PgConsumerListener {
    task_consumer: PgQueueConsumer,
    delay: tokio::time::Duration,
}

impl PgConsumerListener {
    pub fn new(task_consumer: PgQueueConsumer) -> Self {
        Self {
            task_consumer,
            delay: Duration::from_millis(500),
        }
    }

    pub fn listen(self) {
        tokio::spawn(async move {
            loop {
                //TODO: process new messages
                tokio::time::sleep(self.delay).await;
                continue;
            }
        });
    }
}
