use crate::db::models::pg_queue;
use axum::async_trait;
use sea_orm::{prelude::*, QueryOrder};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait};

use super::{TaskQueueConsumer, TaskQueueProducer, TaskQueueReceive, TaskQueueSend};

const DEFAULT_MAIN: &str = "{queue}_svix_v3_main";
//const DEFAULT_DELAYED: &str = "{queue}_svix_delayed";
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
        queue_name: main_queue_name.clone(),
    }));

    let pgq_consumer = PgQueueConsumer {
        pool: opts.pool.clone(),
        queue_name: main_queue_name.clone(),
    };

    let task_consumer = TaskQueueConsumer(Box::new(pgq_consumer.clone()));

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
        //NOTE: as we are building heap. we need to figure out when we will be running this task.
        //If user has provided delay, IE: run after one hour, then we will create a timestamp after one hour
        //and insert it in the database.
        //If user doesn't provide any delay, then we assume, we need to run this task as soon as possible,
        //so, we are taking when_run = now.
        let when_run = match delay {
            None => chrono::Utc::now().timestamp_millis(),
            Some(millis) => chrono::Utc::now().timestamp_millis() + millis.as_millis() as i64,
        };

        let task_obj = pg_queue::ActiveModel {
            id: ActiveValue::NotSet,
            queue_name: ActiveValue::set(self.queue_name.clone()),
            payload: ActiveValue::Set(Some(serde_json::to_value(task).unwrap())),
            when_run: ActiveValue::set(when_run),
            created_at: ActiveValue::Set(chrono::Utc::now()),
        };

        /*TODO: more proper error handling required*/
        pg_queue::Entity::insert(task_obj).exec(&self.pool).await?;
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        todo!()
    }

    //NOTE: once we receive the ack, delete that entry from the database.
    async fn ack(&self, delivery: super::TaskQueueDelivery) -> crate::error::Result<()> {
        pg_queue::Entity::delete_by_id(delivery.id.parse::<i64>().unwrap())
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    //NOTE: re-queue task which were not able to process and received NACK
    //logic is bit simple, should be cheking the reasons and retry count before re-queueing.
    async fn nack(&self, delivery: super::TaskQueueDelivery) -> crate::error::Result<()> {
        self.send(delivery.task, None).await
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
    //Also, this is not safe implementation, only one reader is safe, no locking is involved as of now.
    async fn receive_all(&mut self) -> crate::error::Result<Vec<super::TaskQueueDelivery>> {
        let now_timestamp_millis = chrono::Utc::now().timestamp_millis();

        let tasks = pg_queue::Entity::find()
            .filter(
                Condition::all()
                    .add(pg_queue::Column::WhenRun.lte(now_timestamp_millis))
                    .add(pg_queue::Column::QueueName.eq(&*self.queue_name.clone())),
            )
            .order_by_asc(pg_queue::Column::WhenRun)
            .all(&self.pool)
            .await?;

        let mut t: Vec<super::TaskQueueDelivery> = vec![];

        for task in tasks {
            t.push(super::TaskQueueDelivery {
                id: task.id.to_string(),
                task: serde_json::from_value(task.payload.unwrap()).unwrap(),
            });
        }

        Ok(t)
    }
}

// struct PgConsumerListener {
//     task_consumer: PgQueueConsumer,
//     delay: tokio::time::Duration,
// }

// impl PgConsumerListener {
//     pub fn new(task_consumer: PgQueueConsumer) -> Self {
//         Self {
//             task_consumer,
//             delay: Duration::from_millis(500),
//         }
//     }

//     pub fn listen(mut self) {
//         tokio::spawn(async move {
//             let mut selff = self;
//             loop {
//                 let upcoming_tasks_result = selff.task_consumer.receive_all().await;
//                 match upcoming_tasks_result {
//                     Err(_) => { /*TODO: better handle error */ }
//                     Ok(tasks) => {}
//                 }
//                 tokio::time::sleep(selff.delay).await;
//                 continue;
//             }
//         });
//     }
// }
