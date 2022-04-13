// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::{
    core::types::{ApplicationId, EndpointId, MessageId, MessageStatus},
    v1::{
        endpoints::{
            application::ApplicationOut,
            attempt::{AttemptedMessageOut, MessageAttemptOut},
            endpoint::EndpointOut,
            message::MessageOut,
        },
        utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{
        create_test_app, create_test_endpoint, create_test_message,
        get_msg_attempt_list_and_assert_count,
    },
    get_default_test_config, run_with_retries, start_svix_server, start_svix_server_with_cfg,
    TestClient, TestReceiver,
};

use std::time::Duration;

#[tokio::test]
async fn test_list_attempted_messages() {
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
    let receiver_2 = TestReceiver::start(axum::http::StatusCode::OK);

    let endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap()
        .id;
    let endp_id_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
        .await
        .unwrap()
        .id;

    let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();
    let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
        .await
        .unwrap();
    let msg_3 = create_test_message(&client, &app_id, serde_json::json!({"test": "data3"}))
        .await
        .unwrap();

    let list_1: ListResponse<AttemptedMessageOut> = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/msg/", app_id, endp_id_1),
            StatusCode::OK,
        )
        .await
        .unwrap();
    let list_2: ListResponse<AttemptedMessageOut> = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/msg/", app_id, endp_id_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    for list in [list_1, list_2] {
        assert_eq!(list.data.len(), 3);

        // Assert order
        assert_eq!(list.data[0].msg, msg_3);
        assert_eq!(list.data[1].msg, msg_2);
        assert_eq!(list.data[2].msg, msg_1);
    }
}

#[tokio::test]
async fn test_list_attempts_by_endpoint() {
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "v1AttemptListAttemptsByEndpointTestApp")
        .await
        .unwrap()
        .id;

    let receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
    let receiver_2 = TestReceiver::start(axum::http::StatusCode::OK);

    let endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap()
        .id;
    let endp_id_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
        .await
        .unwrap()
        .id;

    let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();
    let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
        .await
        .unwrap();
    let msg_3 = create_test_message(&client, &app_id, serde_json::json!({"test": "data3"}))
        .await
        .unwrap();

    // And wait at most one second for all attempts to be processed
    run_with_retries(|| async {
        for endp_id in [endp_id_1.clone(), endp_id_2.clone()] {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id),
                    StatusCode::OK,
                )
                .await
                .unwrap();

            if list.data.len() != 3 {
                anyhow::bail!("list len {}, not 3", list.data.len());
            }
        }

        Ok(())
    })
    .await
    .unwrap();

    let list_1: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id_1),
            StatusCode::OK,
        )
        .await
        .unwrap();
    let list_2: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, endp_id_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    for list in [list_1, list_2] {
        let message_ids: Vec<_> = list.data.into_iter().map(|amo| amo.msg_id).collect();
        assert!(message_ids.contains(&msg_1.id));
        assert!(message_ids.contains(&msg_2.id));
        assert!(message_ids.contains(&msg_3.id));
    }

    receiver_1.jh.abort();
    receiver_2.jh.abort();
}

#[tokio::test]
async fn test_message_attempts() {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = (0..2)
        .into_iter()
        .map(|_| Duration::from_millis(1))
        .collect();

    let (client, _jh) = start_svix_server_with_cfg(&cfg);

    for (status_code, msg_status, attempt_count) in [
        // Success
        (StatusCode::OK, MessageStatus::Success, Some(1)),
        // HTTP 400
        (StatusCode::FORBIDDEN, MessageStatus::Fail, None),
        // HTTP 500
        (StatusCode::INTERNAL_SERVER_ERROR, MessageStatus::Fail, None),
    ] {
        let app_id = create_test_app(&client, "app").await.unwrap().id;

        let receiver = TestReceiver::start(status_code);

        let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
            .await
            .unwrap()
            .id;

        let msg = create_test_message(&client, &app_id, serde_json::json!({"test": "data"}))
            .await
            .unwrap();

        let list = get_msg_attempt_list_and_assert_count(
            &client,
            &app_id,
            &msg.id,
            attempt_count.unwrap_or(&cfg.retry_schedule.len() + 1),
        )
        .await
        .unwrap();

        for i in &list.data {
            assert_eq!(i.status, msg_status);
            println!("{} {}", i.response_status_code, status_code);
            assert_eq!(
                i.response_status_code,
                TryInto::<i16>::try_into(status_code.as_u16()).unwrap()
            );
            assert_eq!(i.endpoint_id, endp_id);
        }
        receiver.jh.abort();
    }

    // non-HTTP-related failures:
    let app_id = create_test_app(&client, "app").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);

    // stop receiver before beginning tests:
    receiver.jh.abort();

    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let msg = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();

    let list = get_msg_attempt_list_and_assert_count(
        &client,
        &app_id,
        &msg.id,
        &cfg.retry_schedule.len() + 1,
    )
    .await
    .unwrap();

    for i in &list.data {
        assert_eq!(i.status, MessageStatus::Fail);
        assert_eq!(i.response_status_code, 0);
        assert_eq!(i.endpoint_id, endp_id);
    }
}

async fn get_paginated_attempts_by_endpoint(
    client: &TestClient,
    app_id: &ApplicationId,
    ep_id: &EndpointId,
    query_params: Option<&str>,
) -> anyhow::Result<ListResponse<MessageAttemptOut>> {
    if let Some(query_params) = query_params {
        client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?{}",
                    app_id, ep_id, query_params
                ),
                StatusCode::OK,
            )
            .await
    } else {
        client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app_id, ep_id,),
                StatusCode::OK,
            )
            .await
    }
}

async fn pagination_test_setup(
    client: &TestClient,
) -> (
    ApplicationOut,
    Vec<TestReceiver>,
    Vec<EndpointOut>,
    Vec<MessageOut>,
) {
    // Setup six endpoints and six messages so there's a sufficient number to test pagination
    let app = create_test_app(client, "app1").await.unwrap();

    let mut receivers = Vec::new();
    for _ in 0..6 {
        receivers.push(TestReceiver::start(StatusCode::OK));
    }

    let mut eps = Vec::new();
    for receiver in &receivers {
        eps.push(
            create_test_endpoint(client, &app.id, &receiver.endpoint)
                .await
                .unwrap(),
        );
    }

    let mut messages = Vec::new();
    for i in 1..=6usize {
        messages.push(
            create_test_message(
                client,
                &app.id,
                serde_json::json!({
                    "test": i,
                }),
            )
            .await
            .unwrap(),
        );
    }

    // Wait until all attempts were made
    run_with_retries(|| async {
        for endp_id in eps.iter().map(|ep| &ep.id) {
            let list = get_paginated_attempts_by_endpoint(client, &app.id, endp_id, None)
                .await
                .unwrap();

            if list.data.len() != 6 {
                anyhow::bail!("list len {}, not 6", list.data.len());
            }
        }

        Ok(())
    })
    .await
    .unwrap();

    (app, receivers, eps, messages)
}

#[tokio::test]
async fn test_pagination_by_endpoint() {
    let (client, _jh) = start_svix_server();
    let (app, _receivers, eps, _messages) = pagination_test_setup(&client).await;

    // By endpoint
    for ep in &eps {
        macro_rules! attempts_by_endpoint {
            () => {
                get_paginated_attempts_by_endpoint(&client, &app.id, &ep.id, None).await
            };

            ($query:tt) => {
                get_paginated_attempts_by_endpoint(&client, &app.id, &ep.id, Some(&$query)).await
            };
        }

        let all_attempts = attempts_by_endpoint!().unwrap();

        // Test Limit
        let first_three = attempts_by_endpoint!("limit=3").unwrap();
        assert_eq!(all_attempts.data.len(), 6);
        assert_eq!(&all_attempts.data[0..3], first_three.data.as_slice());

        // Forward iterator
        let query = format!("limit=3&iterator={}", all_attempts.data[2].id);
        let last_three_manual = attempts_by_endpoint!(query).unwrap();

        let query = format!("limit=3&iterator={}", first_three.iterator.unwrap());
        let last_three_iter_field = attempts_by_endpoint!(query).unwrap();

        assert_eq!(last_three_manual.data, last_three_iter_field.data);

        assert_eq!(&all_attempts.data[3..6], last_three_manual.data.as_slice());
        assert!(last_three_manual.done);

        // `prev` iterator
        let query = format!(
            "limit=2&iterator={}",
            last_three_iter_field.prev_iterator.unwrap()
        );
        let two_and_three = attempts_by_endpoint!(query).unwrap();

        assert_eq!(&all_attempts.data[1..3], two_and_three.data.as_slice());
        assert!(!two_and_three.done);

        let query = format!("limit=2&iterator={}", two_and_three.prev_iterator.unwrap());
        let one = attempts_by_endpoint!(query).unwrap();

        assert_eq!(&all_attempts.data[0..1], one.data.as_slice());
        assert!(one.done);

        // `after` field
        let query = format!("after={}", all_attempts.data[3].created_at);
        let first_three_by_time = attempts_by_endpoint!(query).unwrap();
        assert_eq!(
            &all_attempts.data[0..3],
            first_three_by_time.data.as_slice()
        );

        // `before field`
        let query = format!("before={}", all_attempts.data[2].created_at);
        let last_three_by_time = attempts_by_endpoint!(query).unwrap();
        assert_eq!(&all_attempts.data[3..6], last_three_by_time.data.as_slice());
    }
}

/// Adds 5ms to a [`chrono::DateTime`] for testing `before` and `after`
fn add_5ms<T: chrono::TimeZone>(dur: chrono::DateTime<T>) -> chrono::DateTime<T> {
    dur + chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
}

/// Subtracts 5ms to a [`chrono::DateTime`] for testing `before` and `after`
fn sub_5ms<T: chrono::TimeZone>(dur: chrono::DateTime<T>) -> chrono::DateTime<T> {
    dur - chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
}

async fn get_paginated_attempts_by_message(
    client: &TestClient,
    app_id: &ApplicationId,
    ep_id: &MessageId,
    query_params: Option<&str>,
) -> anyhow::Result<ListResponse<MessageAttemptOut>> {
    if let Some(query_params) = query_params {
        client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?{}",
                    app_id, ep_id, query_params
                ),
                StatusCode::OK,
            )
            .await
    } else {
        client
            .get(
                &format!("api/v1/app/{}/attempt/msg/{}/", app_id, ep_id,),
                StatusCode::OK,
            )
            .await
    }
}

#[tokio::test]
async fn test_pagination_by_msg() {
    let (client, _jh) = start_svix_server();
    let (app, _receivers, _eps, messages) = pagination_test_setup(&client).await;

    // By message
    for msg in &messages {
        macro_rules! attempts_by_msg {
            () => {
                get_paginated_attempts_by_message(&client, &app.id, &msg.id, None).await
            };

            ($query:tt) => {
                get_paginated_attempts_by_message(&client, &app.id, &msg.id, Some(&$query)).await
            };
        }

        let all_attempts = attempts_by_msg!().unwrap();

        // Test Limit
        let first_three = attempts_by_msg!("limit=3").unwrap();
        assert_eq!(all_attempts.data.len(), 6);
        assert_eq!(&all_attempts.data[0..3], first_three.data.as_slice());

        // Forward iterator
        let query = format!("limit=3&iterator={}", all_attempts.data[2].id);
        let last_three_manual = attempts_by_msg!(query).unwrap();

        let query = format!("limit=3&iterator={}", first_three.iterator.unwrap());
        let last_three_iter_field = attempts_by_msg!(query).unwrap();

        assert_eq!(last_three_manual.data, last_three_iter_field.data);
        assert_eq!(&all_attempts.data[3..6], last_three_manual.data.as_slice());
        assert!(last_three_manual.done);

        // `prev` iterator
        let query = format!(
            "limit=2&iterator={}",
            last_three_manual.prev_iterator.unwrap()
        );
        let two_and_three = attempts_by_msg!(query).unwrap();

        assert_eq!(&all_attempts.data[1..3], two_and_three.data.as_slice());
        assert!(!two_and_three.done);

        let query = format!("limit=2&iterator={}", two_and_three.prev_iterator.unwrap());
        let one = attempts_by_msg!(query).unwrap();

        assert_eq!(&all_attempts.data[0..1], one.data.as_slice());
        assert!(one.done);

        // Because messages are dispatched so quickly, a different approach than above needs to be tested
        // for checking by time.

        // `after` field
        let query = format!("after={}", sub_5ms(all_attempts.data[5].created_at));
        let all_six_by_time = attempts_by_msg!(query).unwrap();
        assert_eq!(all_attempts.data, all_six_by_time.data);

        let query = format!("after={}", add_5ms(all_attempts.data[0].created_at));
        let none_by_time = attempts_by_msg!(query).unwrap();
        assert!(none_by_time.data.is_empty());

        // `before field`
        let query = format!("before={}", add_5ms(all_attempts.data[0].created_at));
        let all_six_by_time = attempts_by_msg!(query).unwrap();
        assert_eq!(all_attempts.data, all_six_by_time.data);

        let query = format!("before={}", sub_5ms(all_attempts.data[5].created_at));
        let none_by_time = attempts_by_msg!(query).unwrap();
        assert!(none_by_time.data.is_empty());
    }
}

#[tokio::test]
async fn test_pagination_forward_and_back() {
    let (client, _) = start_svix_server();

    let app = create_test_app(&client, "test_app").await.unwrap();

    let receiver = TestReceiver::start(StatusCode::OK);

    let ep = create_test_endpoint(&client, &app.id, &receiver.endpoint)
        .await
        .unwrap();

    let mut messages = Vec::new();
    for i in 1..=28usize {
        messages.push(
            create_test_message(
                &client,
                &app.id,
                serde_json::json!({
                    "test": i,
                }),
            )
            .await
            .unwrap(),
        );
    }

    // Wait until all attempts were made
    run_with_retries(|| async {
        let list: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app.id, ep.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        if list.data.len() != 28 {
            anyhow::bail!("list len {}, not 28", list.data.len());
        }

        Ok(())
    })
    .await
    .unwrap();

    // Go forward
    let mut forward_msgs = Vec::new();
    let mut done = false;
    let mut prev_iterator = None;
    let mut iterator = None;

    while !done {
        let iter_suffix = if let Some(iter) = iterator {
            format!("&iterator={}", iter)
        } else {
            String::new()
        };

        let mut out: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=10{}",
                    app.id, ep.id, iter_suffix
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        forward_msgs.append(&mut out.data);
        done = out.done;
        prev_iterator = out.prev_iterator;
        iterator = out.iterator;
    }

    assert_eq!(forward_msgs.len(), messages.len());

    // Go backwards
    let mut backwards_msgs = Vec::new();
    let mut done = false;

    while !done {
        let iter_suffix = if let Some(iter) = prev_iterator {
            format!("&iterator={}", iter)
        } else {
            String::new()
        };

        let mut out: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=10{}",
                    app.id, ep.id, iter_suffix
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        backwards_msgs.append(&mut out.data);
        done = out.done;
        prev_iterator = out.prev_iterator;
    }

    // Skips the last eight because the prev_iterator is that of the third page of eight going forward
    assert_eq!(backwards_msgs.len(), 20);
    assert_eq!(&forward_msgs[0..10], &backwards_msgs[10..20]);
    assert_eq!(&forward_msgs[10..20], &backwards_msgs[0..10]);
}
