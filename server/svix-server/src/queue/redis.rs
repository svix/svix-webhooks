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
use redis::{
    streams::{
        StreamClaimReply, StreamId, StreamPendingCountReply, StreamPendingReply, StreamRangeReply,
        StreamReadOptions, StreamReadReply,
    },
    Cmd, RedisResult, RedisWrite, ToRedisArgs,
};
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
const DELAYED: &str = "{queue}_svix_delayed";

const LEGACY_MAIN: &str = "svix_queue_main";
const LEGACY_DELAYED: &str = "svix_queue_delayed";

/// After this limit a task should be taken out of the processing queue and rescheduled
const TASK_VALIDITY_DURATION: Duration = Duration::from_secs(45);

/// Consumer group name constant
const WORKERS_GROUP: &str = "svix_workers";
/// Consumer group consumer name constant
const WORKER_CONSUMER: &str = "svix_worker";

/// Special key for XADD command's which generates a real key automatically
const GENERATE_STREAM_ID: &str = "*";
/// Special key for XREADGROUP commands which reads any new messages
const LISTEN_STREAM_ID: &str = ">";

/// Resets the message delivery time, which is used to determine whether a delayed or in-process
/// message should be put back onto the main queue
fn regenerate_key(msg: &str) -> String {
    let task = from_redis_key(msg).task;
    let delivery = TaskQueueDelivery::new(task, None);
    to_redis_key(&delivery)
}

pub async fn new_pair(pool: RedisPool) -> (TaskQueueProducer, TaskQueueConsumer) {
    // Create the stream and consumer group for the MAIN queue should it not already exist
    {
        let mut conn = pool
            .get()
            .await
            .expect("Error retreiving connection from Redis pool");

        let consumer_group_resp: RedisResult<()> = conn
            .query_async(Cmd::xgroup_create_mkstream(MAIN, WORKERS_GROUP, 0i8))
            .await;
    }

    tokio::spawn({
        let pool = pool.clone();
        async move {
            // FIXME: enforce we only have one such worker (locking)
            let batch_size: isize = 50;
            let task_validity_duration =
                chrono::Duration::from_std(TASK_VALIDITY_DURATION).unwrap();

            {
                let mut pool = pool.get().await.unwrap();

                // drain legacy queues:
                migrate_legacy_queues(&mut pool).await;
            }

            loop {
                let mut pool = pool.get().await.unwrap();

                // First look for delayed keys whose time is up and add them to the main qunue
                let timestamp = Utc::now().timestamp();
                let keys: Vec<String> = pool
                    .zrangebyscore_limit(DELAYED, 0isize, timestamp, 0isize, batch_size)
                    .await
                    .unwrap();
                if !keys.is_empty() {
                    // FIXME: needs to be a transaction
                    let keys: Vec<(String, String)> =
                        pool.zpopmin(DELAYED, keys.len() as isize).await.unwrap();
                    let keys: Vec<&str> = keys
                        .iter()
                        .map(|x| &x.0)
                        .map(|x| x.split('|').nth(1).expect("Improper key format"))
                        .collect();

                    for key in keys {
                        let _: () = pool
                            .query_async(Cmd::xadd(
                                MAIN,
                                GENERATE_STREAM_ID,
                                &[(
                                    "data",
                                    serde_json::to_string(key).expect("Serializaion error"),
                                )],
                            ))
                            .await
                            .unwrap();
                    }
                } else {
                    // Wait for half a second before attempting to fetch again if nothing was found
                    sleep(Duration::from_millis(500)).await;
                }

                // Every iteration here also check whether the processing queue has items that
                // should be picked back up
                let mut cmd = Cmd::xpending(MAIN, WORKERS_GROUP);
                // [`Cmd::arg`] returns a mutable reference to the command, and the pool's
                // [`query_async`] takes ownership of the command, so this ugly dropping of the
                // reference is needed for now.
                let _ = cmd.arg("IDLE").arg(45000i32);

                // The `XPENDING` command contains metdata on the start and end IDs plus the count
                // of keys that excepeed the above idle time
                let key_metdata: StreamPendingReply = pool.query_async(cmd).await.unwrap();

                if let StreamPendingReply::Data(data) = key_metdata {
                    let min = data.start_id;
                    let max = data.end_id;
                    let count = data.count;

                    // So use the expanded version of `XPENDING` with a minimum, maximum, and count
                    // to return the exact set of IDs that exceed the given time
                    let mut cmd = Cmd::xpending_count(MAIN, WORKERS_GROUP, min, max, count);
                    let _ = cmd.arg("IDLE").arg(45000i32);

                    let keys: StreamPendingCountReply = pool.query_async(cmd).await.unwrap();
                    let ids: Vec<String> = keys.ids.into_iter().map(|id| id.id).collect();

                    // You can then claim all these IDs to receive the KV pairs associated with each
                    let claimed: StreamClaimReply = pool
                        .query_async(Cmd::xclaim(
                            MAIN,
                            WORKERS_GROUP,
                            WORKER_CONSUMER,
                            450000i32,
                            &ids,
                        ))
                        .await
                        .unwrap();

                    // Acknowledge all the stale ones so the pending queue is cleared
                    let _: RedisResult<()> =
                        pool.query_async(Cmd::xack(MAIN, WORKERS_GROUP, &ids)).await;

                    // And reinsert the map of KV pairs into the MAIN qunue with a new stream ID
                    for StreamId { map, .. } in claimed.ids {
                        let _: RedisResult<()> = pool
                            .query_async(Cmd::xadd(
                                MAIN,
                                GENERATE_STREAM_ID,
                                &map.iter()
                                    .filter_map(|(k, v)| {
                                        if let redis::Value::Data(data) = v {
                                            Some((k.as_str(), data.as_slice()))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<(&str, &[u8])>>(),
                            ))
                            .await;
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
        if let Some(timestamp) = timestamp {
            let delivery = TaskQueueDelivery::new(task, Some(timestamp));
            let key = to_redis_key(&delivery);
            let _: () = pool
                .zadd(DELAYED, key, timestamp.timestamp())
                .await
                .unwrap();
        } else {
            let _: () = pool
                .query_async(Cmd::xadd(
                    MAIN,
                    GENERATE_STREAM_ID,
                    &[("data", serde_json::to_string(&task).unwrap())],
                ))
                .await
                .unwrap();
        }
        tracing::trace!("RedisQueue: event sent > (delay: {:?})", delay);
        Ok(())
    }

    async fn ack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        let mut pool = self.pool.get().await.unwrap();
        let processed: u8 = pool
            .query_async(Cmd::xack(MAIN, WORKERS_GROUP, &[delivery.id.as_str()]))
            .await
            .unwrap();
        if processed != 1 {
            tracing::warn!(
                "Expected to remove 1 from the list, removed {} for {}|{}",
                processed,
                delivery.id,
                serde_json::to_string(&delivery.task).unwrap()
            );
        }
        Ok(())
    }

    async fn nack(&self, delivery: TaskQueueDelivery) -> Result<()> {
        // FIXME: do something else here?
        let mut pool = self.pool.get().await.unwrap();
        let _: () = pool
            .query_async(Cmd::xack(MAIN, WORKERS_GROUP, &[delivery.id.as_str()]))
            .await
            .unwrap();
        tracing::error!(
            "Failed processing msg: {}|{}",
            delivery.id,
            serde_json::to_string(&delivery.task).unwrap()
        );
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
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        let mut pool = self.pool.get().await.unwrap();

        let mut resp: StreamReadReply = pool
            .query_async(Cmd::xread_options(
                &[MAIN],
                &[LISTEN_STREAM_ID],
                &StreamReadOptions::default()
                    .group(WORKERS_GROUP, WORKER_CONSUMER)
                    .count(1),
            ))
            .await
            .unwrap();

        let id = std::mem::take(&mut resp.keys[0].ids[0].id);
        let map = std::mem::take(&mut resp.keys[0].ids[0].map);

        let task: QueueTask = if let Some(redis::Value::Data(data)) = map.get("data") {
            serde_json::from_slice(data).expect("Invalid QueueTask")
        } else {
            panic!("No QueueTask associated with key");
        };

        tracing::trace!("RedisQueue: event recv <");

        Ok(vec![TaskQueueDelivery { id, task }])
    }
}

async fn migrate_legacy_queues(pool: &mut PooledConnection<'_>) {
    migrate_list(pool, LEGACY_MAIN, MAIN).await;
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

    use super::{migrate_list, migrate_sset};

    use crate::{
        cfg::{CacheType, Configuration},
        redis::{PoolLike, PooledConnectionLike, RedisPool},
    };

    async fn get_pool(cfg: Configuration) -> RedisPool {
        match cfg.cache_type {
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
        }
    }

    #[tokio::test]
    async fn test_migrate_list() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;
        let mut pool = pool.get().await.unwrap();

        const TEST_QUEUE: &str = "{queue}_svix_test_queue_list";
        const TEST_LEGACY: &str = "svix_queue_test_list";

        let v = "test-value";

        // delete test queues first, just in case:
        let _: () = pool.del(TEST_QUEUE).await.unwrap();
        let _: () = pool.del(TEST_LEGACY).await.unwrap();

        let _: () = pool.rpush(TEST_LEGACY, &v).await.unwrap();

        let should_be_none: Option<String> = pool.lpop(TEST_QUEUE, None).await.unwrap();
        assert!(should_be_none.is_none());

        migrate_list(&mut pool, TEST_LEGACY, TEST_QUEUE).await;

        let test_key: Option<String> = pool.lpop(TEST_QUEUE, None).await.unwrap();

        assert_eq!(test_key.unwrap(), v);

        let should_be_none: Option<String> = pool.lpop(TEST_LEGACY, None).await.unwrap();
        assert!(should_be_none.is_none());
    }

    #[tokio::test]
    async fn test_migrate_sset() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;
        let mut pool = pool.get().await.unwrap();

        const TEST_QUEUE: &str = "{queue}_svix_test_queue_sset";
        const TEST_LEGACY: &str = "svix_queue_test_sset";

        let v = "test-value";

        // delete test queues first, just in case:
        let _: () = pool.del(TEST_QUEUE).await.unwrap();
        let _: () = pool.del(TEST_LEGACY).await.unwrap();

        let _: () = pool.zadd(TEST_LEGACY, &v, 1isize).await.unwrap();

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();
        assert!(should_be_none.is_empty());

        migrate_sset(&mut pool, TEST_LEGACY, TEST_QUEUE).await;

        let test_key: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();

        assert_eq!(test_key.get(0).unwrap().0, v);

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_LEGACY, 1).await.unwrap();
        assert!(should_be_none.is_empty());
    }
}
