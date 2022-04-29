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
use std::{num::NonZeroUsize, time::Duration};

use axum::async_trait;

use chrono::Utc;
use redis::{RedisWrite, ToRedisArgs};
use svix_ksuid::*;
use tokio::time::sleep;

use crate::{
    error::{Error, Result},
    redis::{PoolLike, PooledConnection, PooledConnectionLike, RedisPool},
};

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

// FIXME: Change unwraps to have our own error type for the queue module entirely

// FIXME: hash-tags here enable clustering but prevent proper sharding of lists
const MAIN: &str = "{queue}_svix_main";
const PROCESSING: &str = "{queue}_svix_processing";
const DELAYED: &str = "{queue}_svix_delayed";

const LEGACY_MAIN: &str = "svix_queue_main";
const LEGACY_PROCESSING: &str = "svix_queue_processing";
const LEGACY_DELAYED: &str = "svix_queue_delayed";

/// After this limit a task should be taken out of the processing queue and rescheduled
const TASK_VALIDITY_DURATION: Duration = Duration::from_secs(45);

pub async fn new_pair(pool: RedisPool) -> (TaskQueueProducer, TaskQueueConsumer) {
    let worker_pool = pool.clone();
    tokio::spawn(async move {
        // FIXME: enforce we only have one such worker (locking)
        let batch_size: isize = 50;
        let task_validity_duration = chrono::Duration::from_std(TASK_VALIDITY_DURATION).unwrap();

        {
            let pool = worker_pool.clone();
            let mut pool = pool.get().await.unwrap();

            // drain legacy queues:
            migrate_legacy_queues(&mut pool).await;
        }

        loop {
            let mut pool = worker_pool.get().await.unwrap();

            let timestamp = Utc::now().timestamp();
            let keys: Vec<String> = pool
                .zrangebyscore_limit(DELAYED, 0isize, timestamp, 0isize, batch_size)
                .await
                .unwrap();
            if !keys.is_empty() {
                // FIXME: needs to be a transaction
                let keys: Vec<(String, String)> =
                    pool.zpopmin(DELAYED, keys.len() as isize).await.unwrap();
                let keys: Vec<String> = keys.into_iter().map(|x| x.0).collect();
                let _: () = pool.rpush(MAIN, keys).await.unwrap();
            } else {
                // Wait for half a second before attempting to fetch again if nothing was found
                sleep(Duration::from_millis(500)).await;
            }

            // Every iteration here also check whether the processing queue has items that should
            // be picked back up
            let keys: Vec<String> = pool.lrange(PROCESSING, 0isize, 1isize).await.unwrap();

            // If the key is older than now, it means we should be processing keys
            let validity_limit =
                KsuidMs::new(Some(Utc::now() - task_validity_duration), None).to_string();

            if !keys.is_empty() && keys[0] <= validity_limit {
                let keys: Vec<String> = pool.lrange(PROCESSING, 0isize, batch_size).await.unwrap();
                for key in keys {
                    if key <= validity_limit {
                        // We use LREM to be sure we only delete the keys we should be deleting
                        tracing::trace!("Pushing back overdue task to queue {}", key);
                        let _: () = pool.rpush(MAIN, &key).await.unwrap();
                        let _: () = pool.lrem(PROCESSING, 1, &key).await.unwrap();
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
    pool: RedisPool,
}

fn to_redis_key(delivery: &TaskQueueDelivery) -> String {
    format!(
        "{}|{}",
        delivery.id,
        serde_json::to_string(&delivery.task).unwrap()
    )
}

fn from_redis_key(key: &str) -> TaskQueueDelivery {
    // Get the first delimiter -> it has to have the |
    let pos = key.find('|').unwrap();
    let id = (&key[..pos]).to_string();
    let task = serde_json::from_str(&key[pos + 1..]).unwrap();
    TaskQueueDelivery { id, task }
}

#[async_trait]
impl TaskQueueSend for RedisQueueProducer {
    async fn send(&self, task: QueueTask, delay: Option<Duration>) -> Result<()> {
        let mut pool = self.pool.get().await.unwrap();
        let timestamp = delay.map(|delay| Utc::now() + chrono::Duration::from_std(delay).unwrap());
        let delivery = TaskQueueDelivery::new(task, timestamp);
        let key = to_redis_key(&delivery);
        if let Some(timestamp) = timestamp {
            let _: () = pool
                .zadd(DELAYED, key, timestamp.timestamp())
                .await
                .unwrap();
        } else {
            let _: () = pool.rpush(MAIN, key).await.unwrap();
        }
        tracing::trace!("RedisQueue: event sent > (delay: {:?})", delay);
        Ok(())
    }

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        let key = to_redis_key(&delivery);
        let mut pool = self.pool.get().await.unwrap();
        let processed: u8 = pool.lrem(PROCESSING, 1, &key).await.unwrap();
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
        let mut pool = self.pool.get().await.unwrap();
        let _: () = pool.lrem(PROCESSING, 1, &key).await.unwrap();
        tracing::error!("Failed processing msg: {}", key);
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn TaskQueueSend> {
        Box::new(self.clone())
    }
}

pub struct RedisQueueConsumer {
    pool: RedisPool,
}

#[async_trait]
impl TaskQueueReceive for RedisQueueConsumer {
    async fn receive(&mut self) -> Result<TaskQueueDelivery> {
        let mut pool = self.pool.get().await.unwrap();
        let mut cmd = redis::cmd("BLMOVE");
        cmd.arg(MAIN)
            .arg(PROCESSING)
            .arg(Direction::Left)
            .arg(Direction::Right)
            .arg(0u8);
        let key: String = pool
            .query_async(cmd)
            .await
            .map_err(|x| Error::Queue(x.to_string()))?;
        tracing::trace!("RedisQueue: event recv <");
        Ok(from_redis_key(&key))
    }
}

async fn migrate_legacy_queues(pool: &mut PooledConnection<'_>) {
    migrate_list(pool, LEGACY_MAIN, MAIN).await;
    migrate_list(pool, LEGACY_PROCESSING, PROCESSING).await;
    migrate_sset(pool, LEGACY_DELAYED, DELAYED).await;
}

async fn migrate_list(pool: &mut PooledConnection<'_>, legacy_queue: &str, queue: &str) {
    let batch_size = 1000;
    loop {
        // Checking for old messages from queue
        let legacy_keys: Vec<String> = pool
            .lpop(legacy_queue, NonZeroUsize::new(batch_size))
            .await
            .unwrap();
        if legacy_keys.is_empty() {
            break;
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );
        let _: () = pool.rpush(queue, legacy_keys).await.unwrap();
    }
}

async fn migrate_sset(pool: &mut PooledConnection<'_>, legacy_queue: &str, queue: &str) {
    let batch_size = 1000;
    loop {
        // Checking for old messages from LEGACY_DELAYED
        let legacy_keys: Vec<(String, f64)> = pool.zpopmin(legacy_queue, batch_size).await.unwrap();

        if legacy_keys.is_empty() {
            break;
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );
        let legacy_keys: Vec<(f64, String)> =
            legacy_keys.into_iter().map(|(x, y)| (y, x)).collect();

        let _: () = pool.zadd_multiple(queue, &legacy_keys).await.unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{cfg::CacheType, redis::PoolLike};

    #[tokio::test]
    async fn test_migrate_legacy_queues() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let pool = match cfg.cache_type {
            CacheType::RedisCluster => {
                let mgr = crate::redis::new_redis_pool_clustered(
                    cfg.redis_dsn.as_ref().unwrap().as_str(),
                )
                .await;
                mgr
            }
            _ => {
                let mgr =
                    crate::redis::new_redis_pool(cfg.redis_dsn.as_ref().unwrap().as_str()).await;
                mgr
            }
        };

        let mut pool = pool.get().await.unwrap();

        let v = "test-value";

        // clean-up:
        let _: () = pool.del(LEGACY_MAIN).await.unwrap();
        let _: () = pool.del(LEGACY_PROCESSING).await.unwrap();
        let _: () = pool.del(LEGACY_DELAYED).await.unwrap();
        let _: () = pool.del(MAIN).await.unwrap();
        let _: () = pool.del(PROCESSING).await.unwrap();
        let _: () = pool.del(DELAYED).await.unwrap();

        let _: () = pool.rpush(super::LEGACY_MAIN, &v).await.unwrap();
        let _: () = pool.rpush(super::LEGACY_PROCESSING, &v).await.unwrap();
        let _: () = pool.zadd(super::LEGACY_DELAYED, &v, 1isize).await.unwrap();

        let main: Option<String> = pool.lpop(super::MAIN, None).await.unwrap();
        let processing: Option<String> = pool.lpop(super::PROCESSING, None).await.unwrap();
        let delayed: Vec<(String, i32)> = pool.zpopmin(super::DELAYED, 1).await.unwrap();

        assert!(main.is_none());
        assert!(processing.is_none());
        assert!(delayed.is_empty());

        super::migrate_legacy_queues(&mut pool).await;

        let main: Option<String> = pool.lpop(super::MAIN, None).await.unwrap();
        let processing: Option<String> = pool.lpop(super::PROCESSING, None).await.unwrap();
        let delayed: Vec<(String, i32)> = pool.zpopmin(super::DELAYED, 1).await.unwrap();

        assert_eq!(main.unwrap(), v);
        assert_eq!(processing.unwrap(), v);
        assert_eq!(delayed.get(0).unwrap().0, v);
    }
}
