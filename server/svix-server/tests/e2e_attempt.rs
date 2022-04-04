// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::{
    core::types::MessageStatus,
    v1::{
        endpoints::attempt::{AttemptedMessageOut, MessageAttemptOut},
        utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message},
    get_default_test_config, run_with_retries, start_svix_server, start_svix_server_with_cfg,
    TestReceiver,
};

use tokio::time::{sleep, Duration};

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

async fn try_message_attempts(
    endpoint: &str,
    msg_status: MessageStatus,
    status_code: i16,
    attempt_count: Option<usize>,
) {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = (0..2)
        .into_iter()
        .map(|_| Duration::from_millis(1))
        .collect();

    let (client, _jh) = start_svix_server_with_cfg(&cfg);

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let endp_id = create_test_endpoint(&client, &app_id, endpoint)
        .await
        .unwrap()
        .id;

    let msg = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();

    for i in &cfg.retry_schedule {
        sleep(*i).await;
    }
    // Give attempts buffer time to complete:
    sleep(Duration::from_millis(50)).await;

    let list = run_with_retries(|| async {
        let list: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/msg/{}", app_id, &msg.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let attempt_count = attempt_count.unwrap_or(cfg.retry_schedule.len() + 1);
        if list.data.len() != attempt_count {
            anyhow::bail!("Attempt count does not match retry_schedule length");
        }
        Ok(list)
    })
    .await
    .unwrap();

    for i in list.data.iter() {
        assert_eq!(i.status, msg_status);
        assert_eq!(i.response_status_code, status_code);
        assert_eq!(i.endpoint_id, endp_id);
    }
}

#[tokio::test]
async fn test_message_attempts() {
    for (status_code, msg_status, attempt_count) in [
        // Success
        (StatusCode::OK, MessageStatus::Success, Some(1)),
        // HTTP 400
        (StatusCode::FORBIDDEN, MessageStatus::Fail, None),
        // HTTP 500
        (StatusCode::INTERNAL_SERVER_ERROR, MessageStatus::Fail, None),
    ] {
        let receiver = TestReceiver::start(status_code);
        try_message_attempts(
            &receiver.endpoint,
            msg_status,
            status_code.as_u16().try_into().unwrap(),
            attempt_count,
        )
        .await;
        receiver.jh.abort();
    }

    // non-HTTP-related failures:
    let receiver = TestReceiver::start(StatusCode::OK);
    receiver.jh.abort();
    try_message_attempts(&receiver.endpoint, MessageStatus::Fail, 0, None).await;
}

#[tokio::test]
async fn test_pagination_by_endpoint() {
    let (client, _jh) = start_svix_server();

    // Setup six endpoints and six messages so there's a sufficient number to test pagination
    let app = create_test_app(&client, "app1").await.unwrap();

    let mut receivers = Vec::new();
    for _ in 0..6 {
        receivers.push(TestReceiver::start(StatusCode::OK));
    }

    let mut eps = Vec::new();
    for receiver in &receivers {
        eps.push(
            create_test_endpoint(&client, &app.id, &receiver.endpoint)
                .await
                .unwrap(),
        );
    }

    let mut messages = Vec::new();
    for i in 1..=6usize {
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
        for endp_id in eps.iter().map(|ep| &ep.id) {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{}/attempt/endpoint/{}/", app.id, endp_id),
                    StatusCode::OK,
                )
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

    // By endpoint
    for ep in &eps {
        let all_attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app.id, ep.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        // Test Limit
        let first_three: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/?limit=3", app.id, ep.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(all_attempts.data.len(), 6);
        assert_eq!(first_three.data.len(), 3);
        assert_eq!(&all_attempts.data[0..3], first_three.data.as_slice());

        // Forward iterator
        let last_three_manual: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=3&iterator={}",
                    app.id, ep.id, all_attempts.data[2].id
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let last_three_iter_field: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=3&iterator={}",
                    app.id,
                    ep.id,
                    first_three.iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(last_three_manual.data, last_three_iter_field.data);

        assert_eq!(last_three_manual.data.len(), 3);
        assert_eq!(&all_attempts.data[3..6], last_three_manual.data.as_slice());
        assert!(last_three_manual.done);

        // `prev` iterator
        let two_and_three: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=2&iterator={}",
                    app.id,
                    ep.id,
                    last_three_manual.prev_iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(two_and_three.data.len(), 2);
        assert_eq!(&all_attempts.data[1..3], two_and_three.data.as_slice());
        assert!(!two_and_three.done);

        let one: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=2&iterator={}",
                    app.id,
                    ep.id,
                    two_and_three.prev_iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(one.data.len(), 1);
        assert_eq!(all_attempts.data[0], one.data[0]);
        assert!(one.done);

        // `after` field
        let first_three_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?after={}",
                    app.id, ep.id, all_attempts.data[3].created_at,
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(first_three_by_time.data.len(), 3);
        assert_eq!(
            &all_attempts.data[0..3],
            first_three_by_time.data.as_slice()
        );

        // `before field`
        let last_three_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?before={}",
                    app.id, ep.id, all_attempts.data[2].created_at,
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(last_three_by_time.data.len(), 3);
        assert_eq!(&all_attempts.data[3..6], last_three_by_time.data.as_slice());
    }
}

#[tokio::test]
async fn test_pagination_by_msg() {
    let (client, _jh) = start_svix_server();

    // Setup six endpoints and six messages so there's a sufficient number to test pagination
    let app = create_test_app(&client, "app1").await.unwrap();

    let mut receivers = Vec::new();
    for _ in 0..6 {
        receivers.push(TestReceiver::start(StatusCode::OK));
    }

    let mut eps = Vec::new();
    for receiver in &receivers {
        eps.push(
            create_test_endpoint(&client, &app.id, &receiver.endpoint)
                .await
                .unwrap(),
        );
    }

    let mut messages = Vec::new();
    for i in 1..=6usize {
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
        for endp_id in eps.iter().map(|ep| &ep.id) {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{}/attempt/endpoint/{}/", app.id, endp_id),
                    StatusCode::OK,
                )
                .await
                .unwrap();

            if list.data.len() != 6 {
                anyhow::bail!("list len {}, not 6", list.data.len());
            }
        }

        Ok(())
    })
    .await
    .unwrap(); // By message
    for msg in &messages {
        let all_attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/msg/{}/", app.id, msg.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        // Test Limit
        let first_three: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/msg/{}/?limit=3", app.id, msg.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(all_attempts.data.len(), 6);
        assert_eq!(first_three.data.len(), 3);

        assert_eq!(&all_attempts.data[0..3], first_three.data.as_slice());

        // Forward iterator
        let last_three_manual: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?limit=3&iterator={}",
                    app.id, msg.id, all_attempts.data[2].id
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let last_three_iter_field: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?limit=3&iterator={}",
                    app.id,
                    msg.id,
                    first_three.iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(last_three_manual.data, last_three_iter_field.data);

        assert_eq!(last_three_manual.data.len(), 3);
        assert_eq!(&all_attempts.data[3..6], last_three_manual.data.as_slice());
        assert!(last_three_manual.done);

        // `prev` iterator
        let two_and_three: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?limit=2&iterator={}",
                    app.id,
                    msg.id,
                    last_three_manual.prev_iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(two_and_three.data.len(), 2);
        assert_eq!(&all_attempts.data[1..3], two_and_three.data.as_slice());
        assert!(!two_and_three.done);

        let one: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?limit=2&iterator={}",
                    app.id,
                    msg.id,
                    two_and_three.prev_iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(one.data.len(), 1);
        assert_eq!(all_attempts.data[0], one.data[0]);
        assert!(one.done);

        // Because messages are dispatched so quickly, a different approach than above needs to be tested
        // for checking by time.

        // `after` field
        let all_six_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?after={}",
                    app.id,
                    msg.id,
                    sub_5ms(all_attempts.data[5].created_at)
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(all_attempts.data, all_six_by_time.data);

        let none_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?after={}",
                    app.id,
                    msg.id,
                    add_5ms(all_attempts.data[0].created_at)
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert!(none_by_time.data.is_empty());

        // `before field`
        let all_six_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?before={}",
                    app.id,
                    msg.id,
                    add_5ms(all_attempts.data[0].created_at),
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(all_attempts.data, all_six_by_time.data);

        let none_by_time: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/msg/{}/?before={}",
                    app.id,
                    msg.id,
                    sub_5ms(all_attempts.data[5].created_at),
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert!(none_by_time.data.is_empty());
    }

    /// Adds 5ms to a [`chrono::DateTime`] for testing `before` and `after`
    fn add_5ms<T: chrono::TimeZone>(dur: chrono::DateTime<T>) -> chrono::DateTime<T> {
        dur + chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
    }

    /// Subtracts 5ms to a [`chrono::DateTime`] for testing `before` and `after`
    fn sub_5ms<T: chrono::TimeZone>(dur: chrono::DateTime<T>) -> chrono::DateTime<T> {
        dur - chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
    }
}
