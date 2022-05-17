// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Configuration-dependent queue tests. This will depend on the set environment variables as with
//! the e2e tests such as to allow testing multiple queue backends via the test script.

use std::{str::FromStr, time::Duration};

use svix_server::{
    core::types::{ApplicationId, EndpointId, MessageAttemptTriggerType, MessageId},
    queue::{new_pair, MessageTask, QueueTask, TaskQueueConsumer, TaskQueueProducer},
};

// Without the `multi_thread` and `worker_threads` directive, the `block_on` call will never return
// and the test will hang.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_many_queue_consumers() {
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");

    // Make 20 producers and 20 consumers using the same configuration
    let mut producers_and_consumers: Vec<(TaskQueueProducer, TaskQueueConsumer)> = Vec::new();
    for _ in 0..20 {
        producers_and_consumers.push(new_pair(cfg.clone()).await);
    }

    // Add 50 test messages with unique message IDs to each producer for a total of 1000 unique
    // messagues
    for (index, (p, _c)) in producers_and_consumers.iter().enumerate() {
        for num in 1..=50 {
            p.send(
                QueueTask::MessageV1(MessageTask {
                    msg_id: MessageId(format!("{}", index * 50 + num)),
                    app_id: ApplicationId("TeatApplicationId".to_owned()),
                    endpoint_id: EndpointId("TestEndpointId".to_owned()),
                    trigger_type: MessageAttemptTriggerType::Manual,
                    attempt_count: 0,
                }),
                None,
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

                loop {
                    tokio::select! {
                        recv = c.receive_all() => {
                            out.append(&mut recv.unwrap());
                        }
                        _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                            break;
                        }
                    }
                }

                out
            })
        }));
    }

    // Create a Vec with all the threads' outputs
    let mut out = Vec::new();
    for jh in join_handles {
        out.append(&mut jh.join().unwrap());
    }

    // Sort it by the message ID
    out.sort_by(|a, b| {
        let a = u16::from_str(a.task.clone().msg_id().as_str()).unwrap();
        let b = u16::from_str(b.task.clone().msg_id().as_str()).unwrap();

        a.cmp(&b)
    });

    // Then assert that all the messages are there

    // This range loop is actually important so it panics if there are less than 1000 messages.
    // With the proposed solution by Clippy, it accepts any smaller vector that's also 1..n.
    // Genreally, however, this lint is actually good practice.
    #[allow(clippy::needless_range_loop)]
    for index in 0..1000 {
        assert_eq!(
            usize::from_str(out[index].task.clone().msg_id().as_str()).unwrap(),
            index + 1
        );
    }
}
