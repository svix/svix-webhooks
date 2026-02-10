// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Configuration-dependent queue tests. This will depend on the set environment variables as with
//! the e2e tests such as to allow testing multiple queue backends via the test script.

use std::{str::FromStr, time::Duration};

use http::StatusCode;
use redis::AsyncCommands as _;
use svix_ksuid::KsuidLike;
use svix_server::{
    cfg::Configuration,
    core::types::{
        ApplicationId, BaseId, EndpointId, MessageAttemptTriggerType, MessageId, OrganizationId,
    },
    queue::{
        MessageTask, QueueTask, TaskQueueConsumer, TaskQueueDelivery, TaskQueueProducer, new_pair,
    },
    redis::RedisManager,
    v1::endpoints::message::MessageOut,
};
use tokio::time::timeout;

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    get_default_test_config, start_svix_server_with_cfg_and_org_id_and_prefix,
};

// TODO: Don't copy this from the Redis queue test directly, place the fn somewhere both can access
async fn get_pool(cfg: &Configuration) -> RedisManager {
    RedisManager::from_queue_backend(&cfg.queue_backend(), cfg.redis_pool_max_size).await
}

fn task_queue_delivery_to_u16(tqd: &TaskQueueDelivery) -> u16 {
    match &*tqd.task {
        QueueTask::HealthCheck => panic!("Health check in test"),
        QueueTask::MessageBatch(batch) => u16::from_str(batch.msg_id.as_str()).unwrap(),
        QueueTask::MessageV1(task) => u16::from_str(task.msg_id.as_str()).unwrap(),
    }
}

async fn test_many_queue_consumers_inner(prefix: &str, delay: Option<Duration>) {
    dotenvy::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");

    // This test assumes an empty queue, so load Redis and delete the test key
    {
        let pool = get_pool(&cfg).await;
        let mut conn = pool.get().await.unwrap();

        let _: () = conn
            .del(format!("{prefix}{{queue}}_svix_v3_main"))
            .await
            .unwrap();
    }

    // Make 20 producers and 20 consumers using the same configuration
    let mut producers_and_consumers: Vec<(TaskQueueProducer, TaskQueueConsumer)> = Vec::new();
    for _ in 0..20 {
        producers_and_consumers.push(new_pair(&cfg, Some(prefix)).await);
    }

    // Add 200 test messages¹ with unique message IDs to each producer for a
    // total of 4000 unique messages
    //
    // ¹ it is important for this number to be no smaller than MAX_MESSAGES in
    //   TaskQueueConsumer::receive_all
    for (index, (p, _c)) in producers_and_consumers.iter().enumerate() {
        for num in 0..200 {
            p.send(
                &QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("{}", index * 200 + num)),
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

                while let Ok(recv) = timeout(
                    Duration::from_secs(1),
                    c.receive_all(Duration::from_secs(5)),
                )
                .await
                {
                    let recv = recv.unwrap();
                    read += recv.len();
                    for r in recv {
                        out.push(task_queue_delivery_to_u16(&r));
                        r.ack().await.unwrap();
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

        if read < 20 {
            panic!("Consumer starved, only read {read} messages");
        }
    }

    // Sort it by the message ID
    out.sort();

    // Then assert that all the messages are there
    assert_eq!(out.len(), 4000);
    for (idx, &num) in out.iter().enumerate() {
        assert_eq!(idx, num as usize);
    }
}

// Without the `multi_thread` and `worker_threads` directive, the `block_on` call will never return
// and the test will hang.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
// run with `cargo test -- --ignored redis` only when redis is up and configured
#[ignore]
async fn test_many_queue_consumers() {
    test_many_queue_consumers_inner("test_many_queue_consumers_", None).await;
}

// Without the `multi_thread` and `worker_threads` directive, the `block_on` call will never return
// and the test will hang.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore]
async fn test_many_queue_consumers_delayed() {
    test_many_queue_consumers_inner(
        "test_many_queue_consumers_delayed_",
        Some(Duration::from_millis(500)),
    )
    .await;
}

#[tokio::test]
#[ignore]
async fn test_redis_streams_dlq() {
    let mut cfg = get_default_test_config();
    cfg.worker_enabled = false;
    cfg.redis_pending_duration_secs = 1;

    let cfg = std::sync::Arc::new(cfg);
    let prefix = svix_ksuid::Ksuid::new(None, None).to_string();

    let pool = get_pool(&cfg).await;
    let mut conn = pool.get().await.unwrap();

    let _: () = conn
        .del(format!("{prefix}{{queue}}_svix_v3_main"))
        .await
        .unwrap();

    let _: () = conn
        .del(format!("{prefix}{{queue}}_svix_dlq"))
        .await
        .unwrap();

    let (client, _jh) = start_svix_server_with_cfg_and_org_id_and_prefix(
        &cfg,
        OrganizationId::new(None, None),
        prefix.clone(),
    )
    .await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    let _message_1: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let (_p, mut c) = new_pair(&cfg, Some(&prefix)).await;

    let wait_time = std::time::Duration::from_millis(1_500);
    for _ in 0..3 {
        let res = c.receive_all(wait_time).await.unwrap();
        assert!(!res.is_empty());
        for j in res {
            j.nack().await.unwrap();
        }
    }

    let res = c.receive_all(wait_time).await.unwrap();
    assert!(res.is_empty());

    tokio::time::sleep(wait_time).await;

    // Redrive
    client
        .post_without_response(
            "/api/v1/admin/redrive-dlq",
            serde_json::Value::Null,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    for _ in 0..3 {
        let res = c.receive_all(wait_time).await.unwrap();
        assert!(!res.is_empty());
        for j in res {
            j.nack().await.unwrap();
        }
    }

    let res = c.receive_all(wait_time).await.unwrap();
    assert!(res.is_empty());
}
