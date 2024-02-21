// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Configuration-dependent queue tests. This will depend on the set environment variables as with
//! the e2e tests such as to allow testing multiple queue backends via the test script.

use std::{str::FromStr, time::Duration};

use svix_server::{
    cfg::{CacheType, Configuration},
    core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
    queue::{
        new_pair, MessageTask, QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer,
    },
    redis::{new_redis_pool, new_redis_pool_clustered, PoolLike, PooledConnectionLike, RedisPool},
};

// TODO: Don't copy this from the Redis queue test directly, place the fn somewhere both can access
pub async fn get_pool(cfg: Configuration) -> RedisPool {
    match cfg.cache_type {
        CacheType::RedisCluster => {
            new_redis_pool_clustered(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await
        }
        _ => new_redis_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await,
    }
}

fn task_queue_delivery_to_u16(tqd: &TaskQueueDelivery) -> u16 {
    match &*tqd.task {
        QueueTask::HealthCheck => panic!("Health check in test"),
        QueueTask::MessageBatch(batch) => u16::from_str(batch.msg_id.as_str()).unwrap(),
        QueueTask::MessageV1(task) => u16::from_str(task.msg_id.as_str()).unwrap(),
    }
}

async fn test_many_queue_consumers_inner(prefix: &str, delay: Option<Duration>) {
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");

    // This test assumes an empty queue, so load Redis and delete the test key
    {
        let pool = get_pool(cfg.clone()).await;
        let mut conn = pool.get().await.unwrap();

        conn.query_async::<()>(redis::Cmd::del(&format!("{prefix}{{queue}}_svix_v3_main")))
            .await
            .unwrap();
    }

    // Make 20 producers and 20 consumers using the same configuration
    let mut producers_and_consumers: Vec<(TaskQueueProducer, TaskQueueConsumer)> = Vec::new();
    for _ in 0..20 {
        producers_and_consumers.push(new_pair(&cfg, Some(prefix)).await);
    }

    // Add 50 test messages with unique message IDs to each producer for a total of 1000 unique
    // messagues
    for (index, (p, _c)) in producers_and_consumers.iter().enumerate() {
        for num in 1..=50 {
            p.send(
                QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("{}", index * 50 + num)),
                    app_id: ApplicationId("TestApplicationId".to_owned()),
                    endpoint_id: EndpointId("TestEndpointId".to_owned()),
                    trigger_type: MessageAttemptTriggerType::Manual,
                    attempt_count: 0,
                }),
                delay,
            )
            .await
            .unwrap();
        }
    }

    let mut join_handles = Vec::new();
    // Producers need to stay alive for the remainder of the test for in-memory queue which uses
    // [`tokio::mpsc`]s, so add them to this [`Vec`]
    let mut producers = Vec::new();

    // Ensure that consumers run on separate OS threads and receive messages until 500ms has passed
    // without any messages
    for (p, mut c) in producers_and_consumers {
        producers.push(p);
        let handle = tokio::runtime::Handle::current();
        join_handles.push(std::thread::spawn(move || {
            handle.block_on(async move {
                let mut out = Vec::new();
                let mut read = 0;

                loop {
                    tokio::select! {
                        recv = c.receive_all() => {
                            let recv = recv.unwrap();
                            read += recv.len();
                            for r in recv {
                                out.push(task_queue_delivery_to_u16(&r));
                                r.ack().await.unwrap();
                            }
                        }
                        _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                            break;
                        }
                    }
                }

                (out, read)
            })
        }));
    }

    // Create a Vec with all the threads' outputs
    let mut out = Vec::new();
    for jh in join_handles {
        let (mut jh_out, read): (Vec<u16>, usize) = jh.join().unwrap();
        out.append(&mut jh_out);

        if read < 5 {
            panic!("Consumer starved, only read {read} messages");
        }
    }

    // Sort it by the message ID
    out.sort();

    // Then assert that all the messages are there

    // This range loop is actually important so it panics if there are less than 1000 messages.
    // With the proposed solution by Clippy, it accepts any smaller vector that's also 1..n.
    // Genreally, however, this lint is actually good practice.
    #[allow(clippy::needless_range_loop)]
    for index in 0..1000 {
        assert_eq!(out[index] as usize, index + 1);
    }
}

// Without the `multi_thread` and `worker_threads` directive, the `block_on` call will never return
// and the test will hang.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_many_queue_consumers() {
    test_many_queue_consumers_inner("test_many_queue_consumers_", None).await;
}

// Without the `multi_thread` and `worker_threads` directive, the `block_on` call will never return
// and the test will hang.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_many_queue_consumers_delayed() {
    test_many_queue_consumers_inner(
        "test_many_queue_consumers_delayed_",
        Some(Duration::from_millis(500)),
    )
    .await;
}
