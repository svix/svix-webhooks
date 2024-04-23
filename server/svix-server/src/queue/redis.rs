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

use omniqueue::backends::{RedisBackend, RedisConfig};
use redis::{AsyncCommands as _, RedisResult};

use super::{QueueTask, TaskQueueConsumer, TaskQueueProducer};
use crate::{
    cfg::{Configuration, QueueType},
    error::Result,
    redis::{PooledConnection, RedisPool},
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

/// Each queue item has a set of KV pairs associated with it, for simplicity a sing key, "data" is
/// used with the entire [`QueueTask`] as the value in serialized JSON
const QUEUE_KV_KEY: &str = "data";

/// Generates a [`TaskQueueProducer`] and a [`TaskQueueConsumer`] backed by Redis.
pub async fn new_pair(
    cfg: &Configuration,
    prefix: Option<&str>,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    new_pair_inner(
        cfg,
        Duration::from_secs(45),
        prefix.unwrap_or_default(),
        MAIN,
        DELAYED,
        DELAYED_LOCK,
    )
    .await
}

/// Runs Redis queue migrations with the given delay schedule. Migrations are run on this schedule
/// such that if an old instance of the server is online after the migrations are made, that no data
/// will be lost assuming the old server is taken offline before the last scheduled delay.
async fn run_migration_schedule(delays: &[Duration], pool: RedisPool) {
    let mut conn = pool
        .get()
        .await
        .expect("Error retrieving connection from Redis pool");

    for delay in delays {
        // drain legacy queues:
        if let Err(e) = migrate_v1_to_v2_queues(&mut conn).await {
            tracing::error!("Error migrating queue: {}", e);
            tokio::time::sleep(*delay).await;
            continue;
        }
        if let Err(e) = migrate_v2_to_v3_queues(&mut conn).await {
            tracing::error!("Error migrating queue: {}", e);
            tokio::time::sleep(*delay).await;
            continue;
        }

        tokio::time::sleep(*delay).await;
    }
}

/// An inner function allowing key constants to be variable for testing purposes
async fn new_pair_inner(
    cfg: &Configuration,
    pending_duration: Duration,
    queue_prefix: &str,
    main_queue_name: &'static str,
    delayed_queue_name: &'static str,
    delayed_lock_name: &'static str,
) -> (TaskQueueProducer, TaskQueueConsumer) {
    let main_queue_name = format!("{queue_prefix}{main_queue_name}");
    let delayed_queue_name = format!("{queue_prefix}{delayed_queue_name}");
    let delayed_lock_name = format!("{queue_prefix}{delayed_lock_name}");

    // This fn is only called from
    // - `queue::new_pair` if the queue type is redis and a DSN is set
    // - redis tests that only makes sense to run with the DSN set
    let dsn = cfg.redis_dsn.as_deref().unwrap();
    let pool = match &cfg.queue_type {
        QueueType::RedisCluster => crate::redis::new_redis_pool_clustered(dsn, cfg).await,
        _ => crate::redis::new_redis_pool(dsn, cfg).await,
    };

    // Create the stream and consumer group for the MAIN queue should it not already exist. The
    // consumer is created automatically upon use so it does not have to be created here.
    {
        let mut conn = pool
            .get()
            .await
            .expect("Error retrieving connection from Redis pool");

        let consumer_group_resp: RedisResult<()> = conn
            .xgroup_create_mkstream(&main_queue_name, WORKERS_GROUP, 0i8)
            .await;

        // If the error is a BUSYGROUP error, then the stream or consumer group already exists. This does
        // not impact functionality, so continue as usual.
        if let Err(e) = consumer_group_resp {
            if !e.to_string().contains("BUSYGROUP") {
                panic!(
                    "error creating consumer group or stream: {:?}, {:?}, {:?}, {:?}, {e:?}",
                    e.kind(),
                    e.detail(),
                    e.code(),
                    e.category()
                )
            };
        }
    }

    // Redis durations are given in integer numbers of milliseconds, so the pending_duration (the
    // time in which a task is allowed to be processing before being restarted) must be converted to
    // one.
    let pending_duration: i64 = pending_duration
        .as_millis()
        .try_into()
        .expect("Pending duration out of bounds");

    // Migrate v1 queues to v2 and v2 queues to v3 on a loop with exponential backoff.
    tokio::spawn(async move {
        let delays = [
            // 11.25 min
            Duration::from_secs(60 * 11 + 15),
            // 22.5 min
            Duration::from_secs(60 * 22 + 30),
            // 45 min
            Duration::from_secs(60 * 45),
            // 1.5 hours
            Duration::from_secs(60 * 30 * 3),
            // 3 hours
            Duration::from_secs(60 * 60 * 3),
            // 6 hours
            Duration::from_secs(60 * 60 * 6),
            // 12 hours
            Duration::from_secs(60 * 60 * 12),
            // 24 hours
            Duration::from_secs(60 * 60 * 24),
        ];

        run_migration_schedule(&delays, pool).await;
    });

    let config = RedisConfig {
        dsn: dsn.to_owned(),
        max_connections: cfg.redis_pool_max_size,
        reinsert_on_nack: false, // TODO
        queue_key: main_queue_name,
        delayed_queue_key: delayed_queue_name,
        delayed_lock_key: delayed_lock_name,
        consumer_group: WORKERS_GROUP.to_owned(),
        consumer_name: WORKER_CONSUMER.to_owned(),
        payload_key: QUEUE_KV_KEY.to_owned(),
        ack_deadline_ms: pending_duration,
    };

    match &cfg.queue_type {
        QueueType::RedisCluster => {
            let (producer, consumer) = RedisBackend::cluster_builder(config)
                .build_pair()
                .await
                .expect("Error initializing redis-cluster queue");

            let producer = TaskQueueProducer::new(producer);
            let consumer = TaskQueueConsumer::new(consumer);
            (producer, consumer)
        }
        _ => {
            let (producer, consumer) = RedisBackend::builder(config)
                .build_pair()
                .await
                .expect("Error initializing redis queue");

            let producer = TaskQueueProducer::new(producer);
            let consumer = TaskQueueConsumer::new(consumer);
            (producer, consumer)
        }
    }
}

fn task_from_redis_key(key: &str) -> Arc<QueueTask> {
    // Get the first delimiter -> it has to have the |
    let pos = key.find('|').unwrap();
    serde_json::from_str(&key[pos + 1..]).unwrap()
}

async fn migrate_v2_to_v3_queues(conn: &mut PooledConnection<'_>) -> Result<()> {
    migrate_list_to_stream(conn, LEGACY_V2_MAIN, MAIN).await?;
    migrate_list_to_stream(conn, LEGACY_V2_PROCESSING, MAIN).await?;

    Ok(())
}

async fn migrate_list_to_stream(
    conn: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        let legacy_keys: Vec<String> = conn
            .lpop(legacy_queue, NonZeroUsize::new(batch_size))
            .await?;
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
            let task = task_from_redis_key(&key);
            let _ = pipe.xadd(
                queue,
                GENERATE_STREAM_ID,
                &[(QUEUE_KV_KEY, serde_json::to_string(&task).unwrap())],
            );
        }

        let _: () = pipe.query_async(conn).await?;
    }
}

async fn migrate_v1_to_v2_queues(conn: &mut PooledConnection<'_>) -> Result<()> {
    migrate_list(conn, LEGACY_V1_MAIN, LEGACY_V2_MAIN).await?;
    migrate_list(conn, LEGACY_V1_PROCESSING, LEGACY_V2_PROCESSING).await?;
    migrate_sset(conn, LEGACY_V1_DELAYED, DELAYED).await?;

    Ok(())
}

async fn migrate_list(
    conn: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        // Checking for old messages from queue
        let legacy_keys: Vec<String> = conn
            .lpop(legacy_queue, NonZeroUsize::new(batch_size))
            .await?;
        if legacy_keys.is_empty() {
            break Ok(());
        }
        tracing::info!(
            "Migrating {} keys from queue {}",
            legacy_keys.len(),
            legacy_queue
        );
        let _: () = conn.rpush(queue, legacy_keys).await?;
    }
}

async fn migrate_sset(
    conn: &mut PooledConnection<'_>,
    legacy_queue: &str,
    queue: &str,
) -> Result<()> {
    let batch_size = 1000;
    loop {
        // Checking for old messages from LEGACY_DELAYED
        let legacy_keys: Vec<(String, f64)> = conn.zpopmin(legacy_queue, batch_size).await?;

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

        let _: () = conn.zadd_multiple(queue, &legacy_keys).await?;
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use assert_matches::assert_matches;
    use chrono::Utc;
    use redis::{streams::StreamReadReply, AsyncCommands as _, Direction};

    use super::{migrate_list, migrate_list_to_stream, migrate_sset, new_pair_inner};
    use crate::{
        cfg::{CacheType, Configuration},
        core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
        queue::{MessageTask, QueueTask, TaskQueueConsumer, TaskQueueProducer},
        redis::RedisPool,
    };

    async fn get_pool(cfg: &Configuration) -> RedisPool {
        match cfg.cache_type {
            CacheType::RedisCluster => {
                crate::redis::new_redis_pool_clustered(cfg.redis_dsn.as_deref().unwrap(), cfg).await
            }
            CacheType::Redis => {
                crate::redis::new_redis_pool(cfg.redis_dsn.as_deref().unwrap(), cfg).await
            }
            _ => {
                panic!("This test should only be run when redis is configured as the queue backend")
            }
        }
    }

    #[tokio::test]
    // run with `cargo test -- --ignored redis` only when redis is up and configured
    #[ignore]
    async fn test_migrate_list() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(&cfg).await;
        let mut pool = pool.get().await.unwrap();

        const TEST_QUEUE: &str = "{queue}_svix_test_queue_list";
        const TEST_LEGACY: &str = "svix_queue_test_list";

        let v = "test-value";

        // delete test queues first, just in case:
        let _: () = pool.del(TEST_QUEUE).await.unwrap();
        let _: () = pool.del(TEST_LEGACY).await.unwrap();

        let _: () = pool.rpush(TEST_LEGACY, v).await.unwrap();

        let should_be_none: Option<String> = pool.lpop(TEST_QUEUE, None).await.unwrap();
        assert_eq!(should_be_none, None);

        migrate_list(&mut pool, TEST_LEGACY, TEST_QUEUE)
            .await
            .unwrap();

        let test_key: Option<String> = pool.lpop(TEST_QUEUE, None).await.unwrap();

        assert_eq!(test_key.unwrap(), v);

        let should_be_none: Option<String> = pool.lpop(TEST_LEGACY, None).await.unwrap();
        assert_eq!(should_be_none, None);
    }

    #[tokio::test]
    #[ignore]
    async fn test_migrate_sset() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(&cfg).await;
        let mut pool = pool.get().await.unwrap();

        const TEST_QUEUE: &str = "{queue}_svix_test_queue_sset";
        const TEST_LEGACY: &str = "svix_queue_test_sset";

        let v = "test-value";

        // delete test queues first, just in case:
        let _: () = pool.del(TEST_QUEUE).await.unwrap();
        let _: () = pool.del(TEST_LEGACY).await.unwrap();

        let _: () = pool.zadd(TEST_LEGACY, v, 1isize).await.unwrap();

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();
        assert_eq!(should_be_none, vec![]);

        migrate_sset(&mut pool, TEST_LEGACY, TEST_QUEUE)
            .await
            .unwrap();

        let test_key: Vec<(String, i32)> = pool.zpopmin(TEST_QUEUE, 1).await.unwrap();

        assert_eq!(test_key.first().unwrap().0, v);

        let should_be_none: Vec<(String, i32)> = pool.zpopmin(TEST_LEGACY, 1).await.unwrap();
        assert_eq!(should_be_none, vec![]);
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
    #[ignore]
    async fn test_idle_period() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(&cfg).await;

        let (p, mut c) = new_pair_inner(
            &cfg,
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
            .expect("Error retrieving connection from Redis pool");
        let keys = conn
            .xread::<_, _, StreamReadReply>(&["{test}_ack"], &[0])
            .await
            .unwrap()
            .keys;
        assert_matches!(keys.as_slice(), []);
    }

    #[tokio::test]
    #[ignore]
    async fn test_ack() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(&cfg).await;

        // Delete the keys used in this test to ensure nothing pollutes the output
        let mut conn = pool
            .get()
            .await
            .expect("Error retrieving connection from Redis pool");
        let _: () = conn
            .del(&[
                "{test}_ack",
                "{test}_ack_delayed",
                "{test}_ack_delayed_lock",
            ])
            .await
            .unwrap();

        let (p, mut c) = new_pair_inner(
            &cfg,
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
        let keys = conn
            .xread::<_, _, StreamReadReply>(&["{test}_ack"], &[0])
            .await
            .unwrap()
            .keys;
        assert_matches!(keys.as_slice(), []);
    }

    #[tokio::test]
    #[ignore]
    async fn test_nack() {
        let cfg = crate::cfg::load().unwrap();

        let (p, mut c) = new_pair_inner(
            &cfg,
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
    #[ignore]
    async fn test_delay() {
        let cfg = crate::cfg::load().unwrap();

        let (p, mut c) = new_pair_inner(
            &cfg,
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

        let [recv2] = c.receive_all().await.unwrap().try_into().unwrap();
        assert_eq!(*recv2.task, mt2);
        recv2.ack().await.unwrap();

        let [recv1] = c.receive_all().await.unwrap().try_into().unwrap();
        assert_eq!(*recv1.task, mt1);
        recv1.ack().await.unwrap();
    }

    fn to_redis_key(id: &str, task: &QueueTask) -> String {
        format!("{id}|{}", serde_json::to_string(task).unwrap())
    }

    #[tokio::test]
    #[ignore]
    async fn test_migrations() {
        let cfg = crate::cfg::load().unwrap();
        let pool = get_pool(&cfg).await;

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
                .del(&[
                    v1_main,
                    v2_main,
                    v3_main,
                    v1_processing,
                    v2_processing,
                    v1_delayed,
                    v2_delayed,
                ])
                .await
                .unwrap();

            // Add v3 consumer group
            let _: () = conn
                .xgroup_create_mkstream(v3_main, super::WORKERS_GROUP, 0i8)
                .await
                .unwrap();

            // Add v1 data
            for num in 1..=10 {
                let _: () = conn
                    .rpush(
                        v1_main,
                        to_redis_key(
                            &num.to_string(),
                            &QueueTask::MessageV1(MessageTask {
                                msg_id: MessageId(format!("TestMessageID{num}")),
                                app_id: ApplicationId("TestApplicationID".to_owned()),
                                endpoint_id: EndpointId("TestEndpointID".to_owned()),
                                trigger_type: MessageAttemptTriggerType::Manual,
                                attempt_count: 0,
                            }),
                        ),
                    )
                    .await
                    .unwrap();
            }

            for num in 11..=15 {
                let _: () = conn
                    .zadd(
                        v1_delayed,
                        to_redis_key(
                            &num.to_string(),
                            &QueueTask::MessageV1(MessageTask {
                                msg_id: MessageId(format!("TestMessageID{num}")),
                                app_id: ApplicationId("TestApplicationID".to_owned()),
                                endpoint_id: EndpointId("TestEndpointID".to_owned()),
                                trigger_type: MessageAttemptTriggerType::Manual,
                                attempt_count: 0,
                            }),
                        ),
                        Utc::now().timestamp() + 2,
                    )
                    .await
                    .unwrap();
            }

            // Move the first five of v1_main to v1_processing
            for _ in 0..5 {
                let _: () = conn
                    .blmove(
                        v1_main,
                        v1_processing,
                        Direction::Left,
                        Direction::Right,
                        0.0,
                    )
                    .await
                    .unwrap();
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
            &cfg,
            Duration::from_secs(5),
            "",
            v3_main,
            v2_delayed,
            v2_delayed_lock,
        )
        .await;

        // 2 second delay on the delayed and pending queue is inserted after main queue, so first
        // the 6-10 should appear, then 1-5, then 11-15

        let mut items = c.receive_all().await.unwrap();
        while items.len() < 15 {
            let more_tasks = c.receive_all().await.unwrap();
            assert!(!more_tasks.is_empty(), "failed to receive all the tasks");
            items.extend(more_tasks);
        }

        let mut items = items.into_iter();
        for num in 6..=10 {
            let recv = items.next().unwrap();
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
            let recv = items.next().unwrap();
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
            let recv = items.next().unwrap();
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

        if items.len() != 0 {
            panic!("received more than the expected number of tasks, rest: {items:?}");
        }
    }
}
