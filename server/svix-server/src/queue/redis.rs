//! Redis stream-based queue implementation
//!
//! # Redis Streams in Brief
//! Redis has a built-in queue called streams. With consumer groups and consumers, messages in this
//! queue will automatically be put into a pending queue when read and deleted when acknowledged.
//!
//! # The Implementation
//! This implementation uses this to allow worker instances to race for messages to dispatch which
//! are then, ideally, acknowledged. If a message is processing for more than 45 seconds, it is
//! reinserted at the back of the queue to be tried again.
//!
//! This implementation uses the following data structures:
//! - A "tasks to be processed" stream - which is what the consumer listens to for tasks.
//!     AKA: Main
//! - A ZSET for delayed tasks with the sort order being the time-to-be-delivered
//!     AKA: Delayed
//!
//! - Tasks in the delayed queue are prefixed with a ksuid so that we can know the timestamp of when
//!   they should be executed.
//!
//! The implementation spawns an additional worker that monitors both the zset delayed tasks and
//! the tasks currently processing. It monitors the zset task set for tasks that should be
//! processed now, and the currently processing queue for tasks that have timed out and should be
//! put back on the main queue.

// This lint warns on `let _: () = ...` which is used throughout this file for Redis commands which
// have generic return types. This is cleaner than the turbofish operator in my opinion.
#![allow(clippy::let_unit_value)]

use std::{num::NonZeroUsize, sync::Arc, time::Duration};

use axum::async_trait;

use chrono::Utc;
use redis::{
    streams::{StreamClaimReply, StreamId, StreamReadOptions, StreamReadReply},
    Cmd, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs,
};
use tokio::time::sleep;

use crate::{
    ctx, err_generic,
    error::Result,
    queue::Acker,
    redis::{PoolLike, PooledConnection, PooledConnectionLike, RedisPool},
};

use super::{
    QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, TaskQueueReceive,
    TaskQueueSend,
};

/// This is the key of the main queue. As a KV store, redis places the entire stream under this key.
/// Confusingly, each message in the queue may have any number of KV pairs.
const MAIN: &str = "{queue}_svix_v3_main";

/// The key for the DELAYED queue in which scheduled messages are placed. This is the same DELAYED
/// queue as v2 of the queue implementation.
const DELAYED: &str = "{queue}_svix_delayed";

/// The key for the lock guarding the delayed queue background task.
const DELAYED_LOCK: &str = "{queue}_svix_delayed_lock";

// v2 KEY CONSTANTS
const LEGACY_V2_MAIN: &str = "{queue}_svix_main";
const LEGACY_V2_PROCESSING: &str = "{queue}_svix_processing";

// v1 KEY CONSTANTS
const LEGACY_V1_MAIN: &str = "svix_queue_main";
const LEGACY_V1_PROCESSING: &str = "svix_queue_processing";
const LEGACY_V1_DELAYED: &str = "svix_queue_delayed";

/// Consumer group name constant -- each consumer group is able to read and acknowledge messages
/// from the queue, and messages are read by all consumer groups.
const WORKERS_GROUP: &str = "svix_workers_group";
/// Consumer group consumer name constant -- consumer groups contain consumers which receive
/// messages in a round-robin manner. Every worker uses the same consumer name such that they race
/// for messages instead of having them evenly distributed.
const WORKER_CONSUMER: &str = "svix_workers_consumer";

/// Special ID for XADD command's which generates a stream ID automatically
const GENERATE_STREAM_ID: &str = "*";
/// Special ID for XREADGROUP commands which reads any new messages
const LISTEN_STREAM_ID: &str = ">";

/// Each queue item has a set of KV pairs associated with it, for simplicity a sing key, "data" is
/// used with the entire [`QueueTask`] as the value in serialized JSON
const QUEUE_KV_KEY: &str = "data";

/// The maximum number of pending messages to reinsert into the queue after becoming stale per loop
const PENDING_BATCH_SIZE: i16 = 1000;

/// Generates a [`TaskQueueProducer`] and a [`TaskQueueConsumer`] backed by Redis.
pub async fn new_pair(
    pool: RedisPool,
    prefix: Option<&str>,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    new_pair_inner(
        pool,
        Duration::from_secs(45),
        prefix.unwrap_or_default(),
        MAIN,
        DELAYED,
        DELAYED_LOCK,
    )
    .await
}

struct StreamAutoclaimReply {
    ids: Vec<StreamId>,
}

impl FromRedisValue for StreamAutoclaimReply {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        // First try the two member array from before Redis 7.0
        match <((), StreamClaimReply)>::from_redis_value(v) {
            Ok(res) => Ok(StreamAutoclaimReply { ids: res.1.ids }),

            // If it's a type error, then try the three member array from Redis 7.0 and after
            Err(e) if e.kind() == redis::ErrorKind::TypeError => {
                <((), StreamClaimReply, ())>::from_redis_value(v)
                    .map(|ok| StreamAutoclaimReply { ids: ok.1.ids })
            }

            // Any other error should be returned as is
            Err(e) => Err(e),
        }
    }
}

async fn background_task_pending(
    pool: RedisPool,
    main_queue_name: String,
    pending_duration: i64,
) -> Result<()> {
    let mut pool = ctx!(pool.get().await)?;

    // Every iteration checks whether the processing queue has items that should be picked back up,
    // claiming them in the process
    let mut cmd = redis::cmd("XAUTOCLAIM");
    cmd.arg(&main_queue_name)
        .arg(WORKERS_GROUP)
        .arg(WORKER_CONSUMER)
        .arg(pending_duration)
        .arg("-")
        .arg("COUNT")
        .arg(PENDING_BATCH_SIZE);

    let StreamAutoclaimReply { ids } = ctx!(pool.query_async(cmd).await)?;

    if !ids.is_empty() {
        let mut pipe = redis::pipe();

        // And reinsert the map of KV pairs into the MAIN qunue with a new stream ID
        for StreamId { map, .. } in &ids {
            let _ = pipe.xadd(
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
            );
        }

        let _: () = ctx!(pool.query_async_pipeline(pipe).await)?;

        // Acknowledge all the stale ones so the pending queue is cleared
        let ids: Vec<_> = ids.iter().map(|wrapped| &wrapped.id).collect();

        let mut pipe = redis::pipe();
        pipe.add_command(Cmd::xack(&main_queue_name, WORKERS_GROUP, &ids));
        pipe.add_command(Cmd::xdel(&main_queue_name, &ids));

        let _: () = ctx!(pool.query_async_pipeline(pipe).await)?;
    } else {
        // Wait for half a second before attempting to fetch again if nothing was found
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

async fn background_task_delayed(
    pool: RedisPool,
    main_queue_name: String,
    delayed_queue_name: String,
    delayed_lock: &str,
) -> Result<()> {
    let batch_size: isize = 50;

    let mut pool = ctx!(pool.get().await)?;

    // There is a lock on the delayed queue processing to avoid race conditions. So first try to
    // acquire the lock should it not already exist. The lock expires after five seconds in case a
    // worker crashes while holding the lock.
    let mut cmd = redis::cmd("SET");
    cmd.arg(delayed_lock)
        .arg(true)
        .arg("NX")
        .arg("PX")
        .arg(5000);
    // WIll be Some("OK") when set or None when not set
    let resp: Option<String> = ctx!(pool.query_async(cmd).await)?;

    if resp.as_deref() == Some("OK") {
        // First look for delayed keys whose time is up and add them to the main qunue
        let timestamp = Utc::now().timestamp();
        let keys: Vec<String> = ctx!(
            pool.zrangebyscore_limit(&delayed_queue_name, 0isize, timestamp, 0isize, batch_size)
                .await
        )?;

        if !keys.is_empty() {
            let tasks: Vec<&str> = keys
                .iter()
                // All information is stored in the key in which the ID and JSON formated task
                // are separated by a `|`. So, take the key, then take the part after the `|`
                .map(|x| x.split('|').nth(1).expect("Improper key format"))
                .collect();

            // For each task, XADD them to the MAIN queue
            let mut pipe = redis::pipe();
            for task in tasks {
                let _ = pipe.xadd(
                    &main_queue_name,
                    GENERATE_STREAM_ID,
                    &[(QUEUE_KV_KEY, task)],
                );
            }
            let _: () = ctx!(pool.query_async_pipeline(pipe).await)?;

            // Then remove the tasks from the delayed queue so they aren't resent
            let _: () = ctx!(pool.query_async(Cmd::zrem(&delayed_queue_name, keys)).await)?;

            // Make sure to release the lock after done processing
            let _: () = ctx!(pool.del(delayed_lock).await)?;
        } else {
            // Make sure to release the lock before sleeping
            let _: () = ctx!(pool.del(delayed_lock).await)?;
            // Wait for half a second before attempting to fetch again if nothing was found
            sleep(Duration::from_millis(500)).await;
        }
    } else {
        // Also sleep half a second if hte lock could not be fetched
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

/// Runs Redis queue migrations with the given delay schedule. Migrations are run on this schedule
/// such that if an old instance of the server is online after the migrations are made, that no data
/// will be lost assuming the old server is taken offline before the last scheduled delay.
async fn run_migration_schedule(delays: &[Duration], pool: RedisPool) {
    for delay in delays {
        let mut pool = match pool.get().await {
            Ok(pool) => pool,
            Err(e) => {
                tracing::error!(
                    "Error fetching Redis connection from pool in migration schedule: {}",
                    e
                );
                tokio::time::sleep(*delay).await;
                continue;
            }
        };

        // drain legacy queues:
        if let Err(e) = migrate_v1_to_v2_queues(&mut pool).await {
            tracing::error!("Error migrating queue: {}", e);
            tokio::time::sleep(*delay).await;
            continue;
        }
        if let Err(e) = migrate_v2_to_v3_queues(&mut pool).await {
            tracing::error!("Error migrating queue: {}", e);
            tokio::time::sleep(*delay).await;
            continue;
        }

        tokio::time::sleep(*delay).await;
    }
}

/// An inner function allowing key constants to be variable for testing purposes
async fn new_pair_inner(
    pool: RedisPool,
    pending_duration: Duration,
    queue_prefix: &str,
    main_queue_name: &'static str,
    delayed_queue_name: &'static str,
    delayed_lock: &'static str,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    let main_queue_name = format!("{queue_prefix}{main_queue_name}");
    let delayed_queue_name = format!("{queue_prefix}{delayed_queue_name}");
    let delayed_lock = format!("{queue_prefix}{delayed_lock}");

    // Create the stream and consumer group for the MAIN queue should it not already exist. The
    // consumer is created automatically upon use so it does not have to be created here.
    {
        let mut conn = pool
            .get()
            .await
            .expect("Error retreiving connection from Redis pool");

        let consumer_group_resp: RedisResult<()> = conn
            .query_async(Cmd::xgroup_create_mkstream(
                &main_queue_name,
                WORKERS_GROUP,
                0i8,
            ))
            .await;

        // If the error is a BUSYGROUP error, then the stream or consumer group already exists. This does
        // not impact functionality, so continue as usual.
        if let Err(e) = consumer_group_resp {
            if !e.to_string().contains("BUSYGROUP") {
                panic!(
                    "error creating consumer group or stream: {:?}, {:?}, {:?}, {:?}, {:?}",
                    e.kind(),
                    e.detail(),
                    e.code(),
                    e.category(),
                    e
                )
            };
        }
    }

    // Redis durationns are given in integer numbers of milliseconds, so the pending_duration (the
    // time in which a task is allowed to be processing before being restarted) must be converted to
    // one.
    let pending_duration: i64 = pending_duration
        .as_millis()
        .try_into()
        .expect("Pending duration out of bounds");

    let worker_pool = pool.clone();
    let mqn = main_queue_name.clone();
    let dqn = delayed_queue_name.clone();

    // Migrate v1 queues to v2 and v2 queues to v3 on a loop with exponential backoff.
    tokio::spawn({
        let pool = pool.clone();

        async move {
            let delays = [
                // 11.25 min
                Duration::from_secs(60 * 11 + 15),
                // 22.5  min
                Duration::from_secs(60 * 22 + 30),
                // 45 min
                Duration::from_secs(60 * 45),
                // 1.5 hours
                Duration::from_secs(60 * 30 * 3),
                // 3  hours
                Duration::from_secs(60 * 60 * 3),
                // 6 hours
                Duration::from_secs(60 * 60 * 6),
                // 12 hours
                Duration::from_secs(60 * 60 * 12),
                // 24 hours
                Duration::from_secs(60 * 60 * 24),
            ];

            run_migration_schedule(&delays, pool).await;
        }
    });

    tokio::spawn({
        let pool = pool.clone();
        let mqn = mqn.clone();
        async move {
            loop {
                if let Err(err) =
                    background_task_delayed(pool.clone(), mqn.clone(), dqn.clone(), &delayed_lock)
                        .await
                {
                    tracing::error!("{}", err);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                };
            }
        }
    });

    tokio::spawn({
        let pool = pool.clone();
        async move {
            loop {
                if let Err(err) =
                    background_task_pending(pool.clone(), mqn.clone(), pending_duration).await
                {
                    tracing::error!("{}", err);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
    });

    let inner = Arc::new(RedisQueueInner {
        pool: worker_pool,
        main_queue_name,
    });

    // Once the background thread has been started, simply return the [`TaskQueueProducer`] and
    // [`TaskQueueConsumer`]
    (
        TaskQueueProducer::Redis(RedisQueueProducer {
            inner: inner.clone(),
            delayed_queue_name: Arc::new(delayed_queue_name),
        }),
        TaskQueueConsumer::Redis(RedisQueueConsumer(inner)),
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

#[derive(Debug)]
pub(super) struct RedisQueueInner {
    pool: RedisPool,
    main_queue_name: String,
}

#[derive(Clone)]
pub struct RedisQueueProducer {
    inner: Arc<RedisQueueInner>,
    delayed_queue_name: Arc<String>,
}

fn to_redis_key(delivery: &TaskQueueDelivery) -> String {
    format!(
        "{}|{}",
        delivery.id,
        serde_json::to_string(&delivery.task).unwrap()
    )
}

fn from_redis_key(key: &str) -> (String, Arc<QueueTask>) {
    // Get the first delimiter -> it has to have the |
    let pos = key.find('|').unwrap();
    let id = key[..pos].to_string();
    let task = serde_json::from_str(&key[pos + 1..]).unwrap();
    (id, task)
}

impl RedisQueueInner {
    async fn send_immedietly(&self, task: Arc<QueueTask>) -> Result<()> {
        let mut pool = ctx!(self.pool.get().await)?;

        let _: () = ctx!(
            pool.query_async(Cmd::xadd(
                &self.main_queue_name,
                GENERATE_STREAM_ID,
                &[(
                    QUEUE_KV_KEY,
                    serde_json::to_string(&task)
                        .map_err(|e| err_generic!("serialization error: {}", e))?,
                )],
            ))
            .await
        )?;

        Ok(())
    }

    /// ACKing the delivery, XACKs the message in the queue so it will no longer be retried
    pub(super) async fn ack(&self, delivery: &TaskQueueDelivery) -> Result<()> {
        let mut pool = ctx!(self.pool.get().await)?;

        let mut pipe = redis::pipe();

        pipe.add_command(Cmd::xack(
            &self.main_queue_name,
            WORKERS_GROUP,
            &[delivery.id.as_str()],
        ))
        .add_command(Cmd::xdel(&self.main_queue_name, &[delivery.id.as_str()]));

        let (processed, deleted): (u8, u8) = ctx!(pool.query_async_pipeline(pipe).await)?;
        if processed != 1 || deleted != 1 {
            tracing::warn!(
                "Expected to remove 1 from the list, acked {}, deleted {}, for {}|{}",
                processed,
                deleted,
                delivery.id,
                serde_json::to_string(&delivery.task)
                    .map_err(|e| err_generic!("serialization error: {}", e))?
            );
        }

        Ok(())
    }

    pub(super) async fn nack(&self, delivery: &TaskQueueDelivery) -> Result<()> {
        tracing::debug!("nack {}", delivery.id);
        self.send_immedietly(delivery.task.clone()).await?;
        self.ack(delivery).await
    }
}

#[async_trait]
impl TaskQueueSend for RedisQueueProducer {
    async fn send(&self, task: Arc<QueueTask>, delay: Option<Duration>) -> Result<()> {
        let timestamp = if let Some(delay) = delay {
            Utc::now()
                + chrono::Duration::from_std(delay)
                    .map_err(|_| err_generic!("Duration out of bounds"))?
        } else {
            tracing::trace!("RedisQueue: event sent (no delay)");
            return self.inner.send_immedietly(task).await;
        };

        // If there's a delay, add it to the DELAYED queue by ZADDING the Redis-key-ified
        // delivery
        let mut pool = ctx!(self.inner.pool.get().await)?;
        let delivery = TaskQueueDelivery::from_arc(
            task.clone(),
            Some(timestamp),
            Acker::Redis(self.inner.clone()),
        );
        let key = to_redis_key(&delivery);
        let delayed_queue_name: &str = &self.delayed_queue_name;
        let _: () = ctx!(
            pool.zadd(delayed_queue_name, key, timestamp.timestamp())
                .await
        )?;

        tracing::trace!("RedisQueue: event sent > (delay: {:?})", delay);
        Ok(())
    }
}

#[derive(Clone)]
pub struct RedisQueueConsumer(Arc<RedisQueueInner>);

#[async_trait]
impl TaskQueueReceive for RedisQueueConsumer {
    async fn receive_all(&mut self) -> Result<Vec<TaskQueueDelivery>> {
        let consumer = self.clone();
        tokio::spawn(async move {
            // TODO: Receive messages in batches so it's not always a Vec with one member
            let mut pool = ctx!(consumer.0.pool.get().await)?;

            // There is no way to make it await a message for unbounded times, so simply block for a short
            // amount of time (to avoid locking) and loop if no messages were retreived
            let resp: StreamReadReply = ctx!(
                pool.query_async(Cmd::xread_options(
                    &[&consumer.0.main_queue_name],
                    &[LISTEN_STREAM_ID],
                    &StreamReadOptions::default()
                        .group(WORKERS_GROUP, WORKER_CONSUMER)
                        .count(1)
                        .block(10_000),
                ))
                .await
            )?;

            if !resp.keys.is_empty() && !resp.keys[0].ids.is_empty() {
                let element = &resp.keys[0].ids[0];
                let id = element.id.clone();
                let map = &element.map;

                let task: QueueTask = if let Some(redis::Value::Data(data)) = map.get("data") {
                    serde_json::from_slice(data).expect("Invalid QueueTask")
                } else {
                    panic!("No QueueTask associated with key");
                };

                tracing::trace!("RedisQueue: event recv <");

                Ok(vec![TaskQueueDelivery {
                    id,
                    task: Arc::new(task),
                    acker: Acker::Redis(consumer.0.clone()),
                }])
            } else {
                Ok(Vec::new())
            }
        })
        .await
        .map_err(|e| err_generic!("task join error {}", e))?
    }
}

async fn migrate_v2_to_v3_queues(pool: &mut PooledConnection<'_>) -> Result<()> {
    migrate_list_to_stream(pool, LEGACY_V2_MAIN, MAIN).await?;
    migrate_list_to_stream(pool, LEGACY_V2_PROCESSING, MAIN).await?;

    Ok(())
}

async fn migrate_list_to_stream(
    pool: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        let legacy_keys: Vec<String> =
            ctx!(pool.lpop(legacy_queue, NonZeroUsize::new(batch_size)).await)?;
        if legacy_keys.is_empty() {
            break Ok(());
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );

        let mut pipe = redis::pipe();
        for key in legacy_keys {
            let (_, task) = from_redis_key(&key);
            let _ = pipe.xadd(
                queue,
                GENERATE_STREAM_ID,
                &[(QUEUE_KV_KEY, serde_json::to_string(&task).unwrap())],
            );
        }

        let _: () = ctx!(pool.query_async_pipeline(pipe).await)?;
    }
}

async fn migrate_v1_to_v2_queues(pool: &mut PooledConnection<'_>) -> Result<()> {
    migrate_list(pool, LEGACY_V1_MAIN, LEGACY_V2_MAIN).await?;
    migrate_list(pool, LEGACY_V1_PROCESSING, LEGACY_V2_PROCESSING).await?;
    migrate_sset(pool, LEGACY_V1_DELAYED, DELAYED).await?;

    Ok(())
}

async fn migrate_list(
    pool: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        // Checking for old messages from queue
        let legacy_keys: Vec<String> =
            ctx!(pool.lpop(legacy_queue, NonZeroUsize::new(batch_size)).await)?;
        if legacy_keys.is_empty() {
            break Ok(());
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );
        let _: () = ctx!(pool.rpush(queue, legacy_keys).await)?;
    }
}

async fn migrate_sset(
    pool: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        // Checking for old messages from LEGACY_DELAYED
        let legacy_keys: Vec<(String, f64)> = ctx!(pool.zpopmin(legacy_queue, batch_size).await)?;

        if legacy_keys.is_empty() {
            break Ok(());
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );
        let legacy_keys: Vec<(f64, String)> =
            legacy_keys.into_iter().map(|(x, y)| (y, x)).collect();

        let _: () = ctx!(pool.zadd_multiple(queue, &legacy_keys).await)?;
    }
}

#[cfg(test)]
pub mod tests {

    use std::{sync::Arc, time::Duration};

    use chrono::Utc;
    use redis::{streams::StreamReadReply, Cmd};

    use super::{
        migrate_list, migrate_list_to_stream, migrate_sset, new_pair_inner, to_redis_key, Direction,
    };

    use crate::{
        cfg::{CacheType, Configuration},
        core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
        queue::{
            redis::RedisQueueInner, Acker, MessageTask, QueueTask, TaskQueueConsumer,
            TaskQueueDelivery, TaskQueueProducer,
        },
        redis::{PoolLike, PooledConnectionLike, RedisPool},
    };

    pub async fn get_pool(cfg: Configuration) -> RedisPool {
        match cfg.cache_type {
            CacheType::RedisCluster => {
                crate::redis::new_redis_pool_clustered(
                    cfg.redis_dsn.as_ref().unwrap().as_str(),
                    &cfg,
                )
                .await
            }
            _ => crate::redis::new_redis_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await,
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

        let _: () = pool.rpush(TEST_LEGACY, v).await.unwrap();

        let should_be_none: Option<String> = pool.lpop(TEST_QUEUE, None).await.unwrap();
        assert!(should_be_none.is_none());

        migrate_list(&mut pool, TEST_LEGACY, TEST_QUEUE)
            .await
            .unwrap();

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

        let _: () = pool.zadd(TEST_LEGACY, v, 1isize).await.unwrap();

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();
        assert!(should_be_none.is_empty());

        migrate_sset(&mut pool, TEST_LEGACY, TEST_QUEUE)
            .await
            .unwrap();

        let test_key: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();

        assert_eq!(test_key.get(0).unwrap().0, v);

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_LEGACY, 1).await.unwrap();
        assert!(should_be_none.is_empty());
    }

    /// Reads and acknowledges all items in the queue with the given name for clearing out entries
    /// from previous test runs
    async fn flush_stale_queue_items(_p: TaskQueueProducer, c: &mut TaskQueueConsumer) {
        loop {
            tokio::select! {
                recv = c.receive_all() => {
                    let recv = recv.unwrap().pop().unwrap();
                    recv.ack().await.unwrap();
                }

                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    break;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_idle_period() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        let (p, mut c) = new_pair_inner(
            pool.clone(),
            Duration::from_millis(100),
            "",
            "{test}_idle_period",
            "{test}_idle_period_delayed",
            "{test}_idle_period_delayed_lock",
        )
        .await;

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
            recv = c.receive_all() => {
                assert_eq!(*recv.unwrap()[0].task, mt);
            }

            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                panic!("`c.receive()` has timed out")
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;

        tokio::select! {
            recv = c.receive_all() => {
                let recv = recv.unwrap().pop().unwrap();
                assert_eq!(*recv.task, mt);
                // Acknowledge so the queue isn't further polluted
                recv.ack().await.unwrap();
            }

            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                panic!("`c.receive()` has timed out")
            }
        }

        // And assert that the task has been deleted
        let mut conn = pool
            .get()
            .await
            .expect("Error retreiving connection from Redis pool");
        assert!(conn
            .query_async::<StreamReadReply>(Cmd::xread(&["{test}_ack"], &[0]))
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    #[tokio::test]
    async fn test_ack() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        // Delete the keys used in this test to ensure nothing polutes the output
        let mut conn = pool
            .get()
            .await
            .expect("Error retreiving connection from Redis pool");
        conn.query_async::<()>(Cmd::del(&[
            "{test}_ack",
            "{test}_ack_delayed",
            "{test}_ack_delayed_lock",
        ]))
        .await
        .unwrap();

        let (p, mut c) = new_pair_inner(
            pool.clone(),
            Duration::from_millis(5000),
            "",
            "{test}_ack",
            "{test}_ack_delayed",
            "{test}_ack_delayed_lock",
        )
        .await;

        let mt = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test2".to_owned()),
            app_id: ApplicationId("test2".to_owned()),
            endpoint_id: EndpointId("test2".to_owned()),
            trigger_type: MessageAttemptTriggerType::Manual,
            attempt_count: 0,
        });
        p.send(mt.clone(), None).await.unwrap();

        let recv = c.receive_all().await.unwrap().pop().unwrap();
        assert_eq!(*recv.task, mt);
        recv.ack().await.unwrap();

        tokio::select! {
            recv = c.receive_all() => {
                panic!("Received unexpected QueueTask {:?}", recv.unwrap()[0].task);
            }

            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
        }

        // And assert that the task has been deleted
        assert!(conn
            .query_async::<StreamReadReply>(Cmd::xread(&["{test}_ack"], &[0]))
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    #[tokio::test]
    async fn test_nack() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        let (p, mut c) = new_pair_inner(
            pool,
            Duration::from_millis(500),
            "",
            "{test}_nack",
            "{test}_nack_delayed",
            "{test}_nack_delayed_lock",
        )
        .await;

        tokio::time::sleep(Duration::from_millis(550)).await;

        flush_stale_queue_items(p.clone(), &mut c).await;

        let mt = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test".to_owned()),
            app_id: ApplicationId("test".to_owned()),
            endpoint_id: EndpointId("test".to_owned()),
            trigger_type: MessageAttemptTriggerType::Manual,
            attempt_count: 0,
        });
        p.send(mt.clone(), None).await.unwrap();

        let recv = c.receive_all().await.unwrap().pop().unwrap();
        assert_eq!(*recv.task, mt);
        recv.nack().await.unwrap();

        tokio::select! {
            recv = c.receive_all() => {
                assert_eq!(*recv.unwrap().pop().unwrap().task, mt);
            }

            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                panic!("Expected QueueTask");
            }
        }
    }

    #[tokio::test]
    async fn test_delay() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        let (p, mut c) = new_pair_inner(
            pool,
            Duration::from_millis(500),
            "",
            "{test}_delay",
            "{test}_delay_delayed",
            "{test}_delay_delayed_lock",
        )
        .await;

        tokio::time::sleep(Duration::from_millis(550)).await;

        flush_stale_queue_items(p.clone(), &mut c).await;

        let mt1 = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test1".to_owned()),
            app_id: ApplicationId("test1".to_owned()),
            endpoint_id: EndpointId("test1".to_owned()),
            trigger_type: MessageAttemptTriggerType::Scheduled,
            attempt_count: 0,
        });
        let mt2 = QueueTask::MessageV1(MessageTask {
            msg_id: MessageId("test2".to_owned()),
            app_id: ApplicationId("test2".to_owned()),
            endpoint_id: EndpointId("test2".to_owned()),
            trigger_type: MessageAttemptTriggerType::Manual,
            attempt_count: 0,
        });

        p.send(mt1.clone(), Some(Duration::from_millis(2000)))
            .await
            .unwrap();
        p.send(mt2.clone(), None).await.unwrap();

        let recv2 = c.receive_all().await.unwrap().pop().unwrap();
        assert_eq!(*recv2.task, mt2);
        recv2.ack().await.unwrap();

        let recv1 = c.receive_all().await.unwrap().pop().unwrap();
        assert_eq!(*recv1.task, mt1);
        recv1.ack().await.unwrap();
    }

    #[tokio::test]
    async fn test_migrations() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(cfg).await;

        // Test queue name constants
        let v1_main = "{test}_migrations_main_v1";
        let v2_main = "{test}_migrations_main_v2";
        let v3_main = "{test}_migrations_main_v3";

        let v1_processing = "{test}_migrations_processing_v1";
        let v2_processing = "{test}_migrations_processing_v2";
        // v3_processing is the stream pending queue for v3_main

        let v1_delayed = "{test}_migrations_delayed_v1";
        let v2_delayed = "{test}_migrations_delayed_v2";
        let v2_delayed_lock = "{test}_migrations_delayed_lock_v2";
        // v3_delayed doesn not yet exist

        {
            let mut conn = pool.get().await.unwrap();

            // Clear test keys
            let _: () = conn
                .query_async(redis::Cmd::del(&[
                    v1_main,
                    v2_main,
                    v3_main,
                    v1_processing,
                    v2_processing,
                    v1_delayed,
                    v2_delayed,
                ]))
                .await
                .unwrap();

            // Add v3 consumer group
            let _: () = conn
                .query_async(redis::Cmd::xgroup_create_mkstream(
                    v3_main,
                    super::WORKERS_GROUP,
                    0i8,
                ))
                .await
                .unwrap();

            // Add v1 data
            for num in 1..=10 {
                let _: () = conn
                    .query_async(redis::Cmd::rpush(
                        v1_main,
                        to_redis_key(&TaskQueueDelivery {
                            id: num.to_string(),
                            task: Arc::new(QueueTask::MessageV1(MessageTask {
                                msg_id: MessageId(format!("TestMessageID{num}")),
                                app_id: ApplicationId("TestApplicationID".to_owned()),
                                endpoint_id: EndpointId("TestEndpointID".to_owned()),
                                trigger_type: MessageAttemptTriggerType::Manual,
                                attempt_count: 0,
                            })),
                            acker: Acker::Redis(Arc::new(RedisQueueInner {
                                pool: pool.clone(),
                                main_queue_name: v1_main.to_owned(),
                            })),
                        }),
                    ))
                    .await
                    .unwrap();
            }

            for num in 11..=15 {
                let _: () = conn
                    .query_async(redis::Cmd::zadd(
                        v1_delayed,
                        to_redis_key(&TaskQueueDelivery {
                            id: num.to_string(),
                            task: Arc::new(QueueTask::MessageV1(MessageTask {
                                msg_id: MessageId(format!("TestMessageID{num}")),
                                app_id: ApplicationId("TestApplicationID".to_owned()),
                                endpoint_id: EndpointId("TestEndpointID".to_owned()),
                                trigger_type: MessageAttemptTriggerType::Manual,
                                attempt_count: 0,
                            })),
                            acker: Acker::Redis(Arc::new(RedisQueueInner {
                                pool: pool.clone(),
                                main_queue_name: v1_main.to_owned(),
                            })),
                        }),
                        Utc::now().timestamp() + 2,
                    ))
                    .await
                    .unwrap();
            }

            // Move the first five of v1_main to v1_processing
            for _ in 0..5 {
                let mut cmd = redis::cmd("BLMOVE");
                cmd.arg(v1_main)
                    .arg(v1_processing)
                    .arg(Direction::Left)
                    .arg(Direction::Right)
                    .arg(0u8);

                let _: () = conn.query_async(cmd).await.unwrap();
            }

            // v1 to v2
            migrate_list(&mut conn, v1_main, v2_main).await.unwrap();
            migrate_list(&mut conn, v1_processing, v2_processing)
                .await
                .unwrap();
            migrate_sset(&mut conn, v1_delayed, v2_delayed)
                .await
                .unwrap();

            // v2 to v3
            migrate_list_to_stream(&mut conn, v2_main, v3_main)
                .await
                .unwrap();
            migrate_list_to_stream(&mut conn, v2_processing, v3_main)
                .await
                .unwrap();
        }

        // Read
        let (_p, mut c) = new_pair_inner(
            pool,
            Duration::from_secs(5),
            "",
            v3_main,
            v2_delayed,
            v2_delayed_lock,
        )
        .await;

        // 2 second delay on the delayed and pending queue is inserted after main queue, so first
        // the 6-10 should appear, then 1-5, then 11-15

        for num in 6..=10 {
            let recv = c.receive_all().await.unwrap().pop().unwrap();
            assert_eq!(
                &*recv.task,
                &QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("TestMessageID{num}")),
                    app_id: ApplicationId("TestApplicationID".to_owned()),
                    endpoint_id: EndpointId("TestEndpointID".to_owned()),
                    trigger_type: MessageAttemptTriggerType::Manual,
                    attempt_count: 0,
                })
            );
            recv.ack().await.unwrap();
        }
        for num in 1..=5 {
            let recv = c.receive_all().await.unwrap().pop().unwrap();
            assert_eq!(
                &*recv.task,
                &QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("TestMessageID{num}")),
                    app_id: ApplicationId("TestApplicationID".to_owned()),
                    endpoint_id: EndpointId("TestEndpointID".to_owned()),
                    trigger_type: MessageAttemptTriggerType::Manual,
                    attempt_count: 0,
                })
            );
            recv.ack().await.unwrap();
        }
        for num in 11..=15 {
            let recv = c.receive_all().await.unwrap().pop().unwrap();
            assert_eq!(
                &*recv.task,
                &QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("TestMessageID{num}")),
                    app_id: ApplicationId("TestApplicationID".to_owned()),
                    endpoint_id: EndpointId("TestEndpointID".to_owned()),
                    trigger_type: MessageAttemptTriggerType::Manual,
                    attempt_count: 0,
                })
            );
            recv.ack().await.unwrap();
        }
    }
}
