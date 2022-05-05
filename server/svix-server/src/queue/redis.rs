// Redis queue implementation
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
        StreamClaimReply, StreamId, StreamPendingCountReply, StreamReadOptions, StreamReadReply,
    },
    Cmd, RedisResult, RedisWrite, ToRedisArgs,
};
use tokio::time::sleep;

use crate::{
    error::Result,
    redis::{PoolLike, PooledConnection, PooledConnectionLike, RedisPool},
};

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

// FIXME: Change unwraps to have our own error type for the queue module entirely

const MAIN: &str = "svix_{queue}_v3_main";

// TODO: Stream based delayed queue
// const DELAYED: &str = "svix_queue_v3_delayed";

// FIXME: hash-tags here enable clustering but prevent proper sharding of lists
const LEGACY_MAIN: &str = "{queue}_svix_main";
const LEGACY_PROCESSING: &str = "{queue}_svix_processing";
const LEGACY_DELAYED: &str = "{queue}_svix_delayed";

const LEGACY_LEGACY_MAIN: &str = "svix_queue_main";
const LEGACY_LEGACY_PROCESSING: &str = "svix_queue_processing";
const LEGACY_LEGACY_DELAYED: &str = "svix_queue_delayed";

/// Consumer group name constant
const WORKERS_GROUP: &str = "svix_workers_group";
/// Consumer group consumer name constant
const WORKER_CONSUMER: &str = "svix_workers_consumer";

/// Special key for XADD command's which generates a real key automatically
const GENERATE_STREAM_ID: &str = "*";
/// Special key for XREADGROUP commands which reads any new messages
const LISTEN_STREAM_ID: &str = ">";

pub async fn new_pair(pool: RedisPool) -> (TaskQueueProducer, TaskQueueConsumer) {
    new_pair_inner(pool, Duration::from_secs(45), MAIN).await
}

async fn new_pair_inner(
    pool: RedisPool,
    pending_duration: Duration,
    main_queue_name: &'static str,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    // Create the stream and consumer group for the MAIN queue should it not already exist
    {
        let mut conn = pool
            .get()
            .await
            .expect("Error retreiving connection from Redis pool");

        // TODO: Check error type
        let consumer_group_resp: RedisResult<()> = conn
            .query_async(Cmd::xgroup_create_mkstream(
                &main_queue_name,
                WORKERS_GROUP,
                0i8,
            ))
            .await;
    }

    let pending_duration: i64 = pending_duration
        .as_millis()
        .try_into()
        .expect("Pending duration out of bounds");

    let worker_pool = pool.clone();
    tokio::spawn(async move {
        // FIXME: enforce we only have one such worker (locking)
        let batch_size: isize = 50;
        let pool = worker_pool;

        {
            let mut pool = pool.get().await.unwrap();

            // drain legacy queues:
            migrate_v1_to_v2_queues(&mut pool).await;
            migrate_v2_to_v3_queues(&mut pool).await;
        }

        loop {
            let mut pool = pool.get().await.unwrap();

            // First look for delayed keys whose time is up and add them to the main qunue
            let timestamp = Utc::now().timestamp();
            let keys: Vec<String> = pool
                .zrangebyscore_limit(LEGACY_DELAYED, 0isize, timestamp, 0isize, batch_size)
                .await
                .unwrap();
            if !keys.is_empty() {
                // FIXME: needs to be a transaction
                let keys: Vec<(String, String)> = pool
                    .zpopmin(LEGACY_DELAYED, keys.len() as isize)
                    .await
                    .unwrap();
                let keys: Vec<&str> = keys
                    .iter()
                    .map(|x| &x.0)
                    .map(|x| x.split('|').nth(1).expect("Improper key format"))
                    .collect();

                for key in keys {
                    let _: () = pool
                        .query_async(Cmd::xadd(
                            &main_queue_name,
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
            let mut cmd = redis::cmd("XPENDING");
            let _ = cmd
                .arg(&main_queue_name)
                .arg(WORKERS_GROUP)
                .arg("IDLE")
                .arg(pending_duration)
                .arg("-")
                .arg("+")
                .arg(1000);

            let keys: StreamPendingCountReply = pool.query_async(cmd).await.unwrap();

            let ids: Vec<String> = keys.ids.into_iter().map(|id| id.id).collect();

            if !ids.is_empty() {
                // You can then claim all these IDs to receive the KV pairs associated with each
                let claimed: StreamClaimReply = pool
                    .query_async(Cmd::xclaim(
                        &main_queue_name,
                        WORKERS_GROUP,
                        WORKER_CONSUMER,
                        pending_duration,
                        &ids,
                    ))
                    .await
                    .unwrap();

                // Acknowledge all the stale ones so the pending queue is cleared
                let _: RedisResult<()> = pool
                    .query_async(Cmd::xack(&main_queue_name, WORKERS_GROUP, &ids))
                    .await;

                // And reinsert the map of KV pairs into the MAIN qunue with a new stream ID
                for StreamId { map, .. } in claimed.ids {
                    let _: RedisResult<()> = pool
                        .query_async(Cmd::xadd(
                            &main_queue_name,
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
    });
    (
        TaskQueueProducer(Box::new(RedisQueueProducer {
            pool: pool.clone(),
            main_queue_name,
        })),
        TaskQueueConsumer(Box::new(RedisQueueConsumer {
            pool,
            main_queue_name,
        })),
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
    main_queue_name: &'static str,
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
                .zadd(LEGACY_DELAYED, key, timestamp.timestamp())
                .await
                .unwrap();
        } else {
            let _: () = pool
                .query_async(Cmd::xadd(
                    &self.main_queue_name,
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
            .query_async(Cmd::xack(
                self.main_queue_name,
                WORKERS_GROUP,
                &[delivery.id.as_str()],
            ))
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
            .query_async(Cmd::xack(
                self.main_queue_name,
                WORKERS_GROUP,
                &[delivery.id.as_str()],
            ))
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

#[derive(Debug, Clone)]
pub struct RedisQueueConsumer {
    pool: RedisPool,
    main_queue_name: &'static str,
}

#[async_trait]
impl TaskQueueReceive for RedisQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        let mut pool = self.pool.get().await.unwrap();

        let mut resp = loop {
            let resp: StreamReadReply = pool
                .query_async(Cmd::xread_options(
                    &[self.main_queue_name],
                    &[LISTEN_STREAM_ID],
                    &StreamReadOptions::default()
                        .group(WORKERS_GROUP, WORKER_CONSUMER)
                        .count(1)
                        .block(500),
                ))
                .await
                .unwrap();

            if !resp.keys.is_empty() {
                if !resp.keys[0].ids.is_empty() {
                    break resp;
                }
            }
        };

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

async fn migrate_v2_to_v3_queues(pool: &mut PooledConnection<'_>) {
    migrate_list_to_stream(pool, LEGACY_MAIN, MAIN).await;
}

async fn migrate_list_to_stream(pool: &mut PooledConnection<'_>, legacy_queue: &str, queue: &str) {
    let batch_size = 1000;
    loop {
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

        for key in legacy_keys {
            let delivery = from_redis_key(&key);
            let _: () = pool
                .query_async(Cmd::xadd(
                    queue,
                    GENERATE_STREAM_ID,
                    &[("data", serde_json::to_string(&delivery.task).unwrap())],
                ))
                .await
                .unwrap();
        }
    }
}

async fn migrate_v1_to_v2_queues(pool: &mut PooledConnection<'_>) {
    migrate_list(pool, LEGACY_LEGACY_MAIN, LEGACY_MAIN).await;
    migrate_list(pool, LEGACY_LEGACY_PROCESSING, LEGACY_LEGACY_PROCESSING).await;
    migrate_sset(pool, LEGACY_LEGACY_DELAYED, LEGACY_DELAYED).await;
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

    use std::time::Duration;

    use super::{migrate_list, migrate_sset, new_pair_inner};

    use crate::{
        cfg::{CacheType, Configuration},
        core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
        queue::{MessageTask, QueueTask, TaskQueueConsumer, TaskQueueProducer},
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

    /// Reads and acknowledges all items in the queue with the given name for clearing out entries
    /// from previous test runs
    async fn flush_stale_queue_items(p: TaskQueueProducer, c: &mut TaskQueueConsumer) {
        'outer: loop {
            tokio::select! {
                recv = c.receive() => {
                    println!("HERE");
                    let recv = recv.unwrap();
                    p.ack(recv).await.unwrap();
                }

                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    println!("BREAK");
                    break 'outer;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_idle_period() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        let (p, mut c) =
            new_pair_inner(pool, Duration::from_millis(100), "{test}_idle_period").await;

		tokio::time::sleep(Duration::from_millis(150)).await;
        flush_stale_queue_items(p.clone(), &mut c).await;

        let mt = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test".to_owned()),
            app_id: ApplicationId("test".to_owned()),
            endpoint_id: EndpointId("test".to_owned()),
            trigger_type: MessageAttemptTriggerType::Manual,
            attempt_count: 0,
        });
        p.send(mt.clone(), None).await.unwrap();

        tokio::select! {
            recv = c.receive() => {
                assert_eq!(recv.unwrap().task, mt);
            }

            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                panic!("`c.receive()` has timed out")
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;

        tokio::select! {
            recv = c.receive() => {
                let recv = recv.unwrap();
                assert_eq!(recv.task, mt);
                // Acknowledge so the queue isn't further polluted
                p.ack(recv).await.unwrap();
            }

            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                panic!("`c.receive()` has timed out")
            }
        }
    }

    #[tokio::test]
    async fn test_ack() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        let (p, mut c) = new_pair_inner(pool, Duration::from_millis(500), "{test}_ack").await;

		tokio::time::sleep(Duration::from_millis(550)).await;

        flush_stale_queue_items(p.clone(), &mut c).await;

        let mt = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test2".to_owned()),
            app_id: ApplicationId("test2".to_owned()),
            endpoint_id: EndpointId("test2".to_owned()),
            trigger_type: MessageAttemptTriggerType::Manual,
            attempt_count: 0,
        });
        p.send(mt.clone(), None).await.unwrap();

        let recv = c.receive().await.unwrap();
        assert_eq!(recv.task, mt);
        p.ack(recv).await.unwrap();

        tokio::select! {
            recv = c.receive() => {
                panic!("Received unexpected QueueTask {:?}", recv.unwrap().task);
            }

            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
        }
    }
}
