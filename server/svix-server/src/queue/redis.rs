/// Redis queue implementation
///
/// This implementation uses the following data structures:
/// - A "tasks to be processed" queue - which is what the consumer listens to for tasks.
///     AKA: Main
/// - A "tasks currently processing" queue - which are for tasks that are currently being handled.
///     AKA: Processing
/// - A ZSET for delayed tasks with the sort order being the time-to-be-delivered
///     AKA: Delayed
///
/// - Tasks in the queues are prefixed with a ksuid so that we can know the timestamp of when they
/// should be executed.
///
/// The implementation spawns an additional worker that monitors both the zset delayed tasks and
/// the tasks currently processing. It monitors the zset task set for tasks that should be
/// processed now, and the currently processing queue for tasks that have timed out and should be
/// put back on the main queue.
use std::time::Duration;

use axum::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::Utc;
use redis::{AsyncCommands, RedisWrite, ToRedisArgs};
use svix_ksuid::*;
use tokio::time::sleep;

use crate::error::{Error, Result};

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

// FIXME: Change unwraps to have our own error type for the queue module entirely

const MAIN: &str = "svix_queue_main";
const PROCESSING: &str = "svix_queue_processing";
const DELAYED: &str = "svix_queue_delayed";

/// After this limit a task should be taken out of the processing queue and rescheduled
const TASK_VALIDITY_DURATION: Duration = Duration::from_secs(45);

pub async fn new_pair(
    pool: Pool<RedisConnectionManager>,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    let worker_pool = pool.clone();
    tokio::spawn(async move {
        // FIXME: enforce we only have one such worker (locking)
        let batch_size: isize = 50;
        let task_validity_duration = chrono::Duration::from_std(TASK_VALIDITY_DURATION).unwrap();

        let pool = worker_pool;
        loop {
            let mut pool = pool.get().await.expect("Error getting Redis connection");
            let timestamp = Utc::now().timestamp();
            let keys: Vec<String> = pool
                .zrangebyscore_limit(DELAYED, 0isize, timestamp, 0isize, batch_size)
                .await
                .expect("Error getting keys from Redis");
            if !keys.is_empty() {
                // FIXME: needs to be a transaction
                let keys: Vec<(String, String)> = pool
                    .zpopmin(DELAYED, keys.len() as isize)
                    .await
                    .expect("Error popping keys from Redis");
                let keys: Vec<String> = keys.into_iter().map(|x| x.0).collect();
                let _: () = pool
                    .rpush(MAIN, keys)
                    .await
                    .expect("Error pushing keys to Redis");
            } else {
                // Wait for half a second before attempting to fetch again if nothing was found
                sleep(Duration::from_millis(500)).await;
            }

            // Every iteration here also check whether the processing queue has items that should
            // be picked back up
            let keys: Vec<String> = pool
                .lrange(PROCESSING, 0isize, 1isize)
                .await
                .expect("Error getting keys from Redis");

            // If the key is older than now, it means we should be processing keys
            let validity_limit =
                KsuidMs::new(Some(Utc::now() - task_validity_duration), None).to_string();
            if !keys.is_empty() && keys[0] <= validity_limit {
                let keys: Vec<String> = pool
                    .lrange(PROCESSING, 0isize, batch_size)
                    .await
                    .expect("Error getting keys from Redis");
                for key in keys {
                    if key <= validity_limit {
                        // We use LREM to be sure we only delete the keys we should be deleting
                        tracing::trace!("Pushing back overdue task to queue {}", key);
                        let _: () = pool
                            .rpush(MAIN, &key)
                            .await
                            .expect("Error pushing key to Redis");
                        let _: () = pool
                            .lrem(PROCESSING, 1, &key)
                            .await
                            .expect("Error removing key from Redis");
                    }
                }
            }
        }
    });
    (
        TaskQueueProducer(Box::new(RedisQueueProducer { pool: pool.clone() })),
        TaskQueueConsumer(Box::new(RedisQueueConsumer { pool })),
    )
}

/// Enum for the LEFT | RIGHT args used by some commands
pub enum Direction {
    Left,
    Right,
}

impl ToRedisArgs for Direction {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let s: &[u8] = match self {
            Direction::Left => b"LEFT",
            Direction::Right => b"RIGHT",
        };
        out.write_arg(s);
    }
}

#[derive(Clone)]
pub struct RedisQueueProducer {
    pool: Pool<RedisConnectionManager>,
}

fn to_redis_key(delivery: &TaskQueueDelivery) -> String {
    format!(
        "{}|{}",
        delivery.id,
        serde_json::to_string(&delivery.task).expect("Error serializing task")
    )
}

fn from_redis_key(key: &str) -> TaskQueueDelivery {
    // Get the first delimiter -> it has to have the |
    let pos = key.find('|').expect("Error finding '|' delimiter");
    let id = (&key[..pos]).to_string();
    let task = serde_json::from_str(&key[pos + 1..]).expect("Error deserializing task");
    TaskQueueDelivery { id, task }
}

#[async_trait]
impl TaskQueueSend for RedisQueueProducer {
    async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let mut pool = self
            .pool
            .get()
            .await
            .expect("Error getting Redis connection");
        let timestamp = delay.map(|delay| {
            Utc::now() + chrono::Duration::from_std(delay).expect("Error parsing delay")
        });
        let delivery = TaskQueueDelivery::new(task, timestamp);
        let key = to_redis_key(&delivery);
        if let Some(timestamp) = timestamp {
            let _: () = pool
                .zadd(DELAYED, key, timestamp.timestamp())
                .await
                .expect("Error adding key to Redis");
        } else {
            let _: () = pool
                .rpush(MAIN, key)
                .await
                .expect("Error pushing key to Redis");
        }
        tracing::trace!("RedisQueue: event sent > (delay: {:?})", delay);
        Ok(())
    }

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        let key = to_redis_key(&delivery);
        let mut pool = self
            .pool
            .get()
            .await
            .expect("Error getting Redis connection");
        let processed: u8 = pool
            .lrem(PROCESSING, 1, &key)
            .await
            .expect("Error removing key from Redis");
        if processed != 1 {
            tracing::warn!(
                "Expected to remove 1 from the list, removed {} for {}",
                processed,
                key
            );
        }
        Ok(())
    }

    async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        // FIXME: do something else here?
        let key = to_redis_key(&delivery);
        let mut pool = self
            .pool
            .get()
            .await
            .expect("Error getting Redis connection");
        let _: () = pool
            .lrem(PROCESSING, 1, &key)
            .await
            .expect("Error removing key from Redis");
        tracing::error!("Failed processing msg: {}", key);
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        Box::new(self.clone())
    }
}

pub struct RedisQueueConsumer {
    pool: Pool<RedisConnectionManager>,
}

#[async_trait]
impl TaskQueueReceive for RedisQueueConsumer {
    async fn receive(&mut self) -> Result<TaskQueueDelivery> {
        let mut pool = self
            .pool
            .get()
            .await
            .expect("Error getting Redis connection");
        let key: String = redis::cmd("BLMOVE")
            .arg(MAIN)
            .arg(PROCESSING)
            .arg(Direction::Left)
            .arg(Direction::Right)
            .arg(0u8)
            .query_async(&mut *pool)
            .await
            .map_err(|x| Error::Queue(x.to_string()))?;
        tracing::trace!("RedisQueue: event recv <");
        Ok(from_redis_key(&key))
    }
}
