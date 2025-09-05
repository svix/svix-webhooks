// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use reqwest::StatusCode;
use serde_json::json;
use svix_server::{
    core::types::{EndpointUid, MessageStatus},
    v1::{
        endpoints::{
            attempt::{EndpointMessageOut, MessageAttemptOut},
            endpoint::{EndpointIn, EndpointOut},
        },
        utils::ListResponse,
    },
};

use crate::utils::{
    common_calls::{
        create_test_app, create_test_endpoint, create_test_message, create_test_msg_with,
        endpoint_in, get_msg_attempt_list_and_assert_count,
    },
    get_default_test_config, run_with_retries, start_svix_server, start_svix_server_with_cfg,
    TestReceiver,
};

#[tokio::test]
async fn test_expunge_attempt_response_body() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let sensitive_response_json = serde_json::json!({"sensitive":"data"});
    let mut receiver = TestReceiver::start_with_body(
        axum::http::StatusCode::OK,
        axum::Json(sensitive_response_json.clone()),
    );

    let endpoint_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let msg_id = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap()
        .id;

    receiver.data_recv.recv().await;

    let attempt = run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}/"),
                StatusCode::OK,
            )
            .await
            .unwrap();
        if attempts.data.len() != 1 {
            anyhow::bail!("list len {}, not 1", attempts.data.len());
        }
        Ok(attempts.data[0].clone())
    })
    .await
    .unwrap();

    let attempt_response: serde_json::Value = serde_json::from_str(&attempt.response).unwrap();
    assert_eq!(sensitive_response_json, attempt_response);

    let attempt_id = &attempt.id;
    client
        .delete(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content/"),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let attempt: MessageAttemptOut = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!("EXPUNGED", &attempt.response);
}

#[tokio::test]
async fn test_list_attempted_messages() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
    let receiver_2 = TestReceiver::start(axum::http::StatusCode::OK);

    let endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap()
        .id;

    // Let's have an endpoint with a UID too
    let mut endp2 = endpoint_in(&receiver_2.endpoint);
    endp2.uid = Some(EndpointUid("test".to_owned()));
    let endp_id_2 = client
        .post::<EndpointIn, EndpointOut>(
            &format!("api/v1/app/{app_id}/endpoint/"),
            endp2,
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;

    let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();
    let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
        .await
        .unwrap();
    let msg_3 = create_test_msg_with(
        &client,
        &app_id,
        serde_json::json!({"test": "data3"}),
        "balloon.popped",
        ["news"],
    )
    .await;

    run_with_retries(|| async {
        let list_1: ListResponse<EndpointMessageOut> = client
            .get(
                &format!("api/v1/app/{app_id}/endpoint/{endp_id_1}/msg/"),
                StatusCode::OK,
            )
            .await
            .unwrap();
        let list_2: ListResponse<EndpointMessageOut> = client
            .get(
                &format!("api/v1/app/{app_id}/endpoint/{endp_id_2}/msg/"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let list_2_uid: ListResponse<EndpointMessageOut> = client
            .get(
                &format!("api/v1/app/{app_id}/endpoint/{}/msg/", "test"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        for list in [list_1, list_2, list_2_uid] {
            if list.data.len() != 3 {
                anyhow::bail!("list len {}, not 3", list.data.len());
            }

            assert!(list.data.iter().any(|x| x.msg == msg_1));
            assert!(list.data.iter().any(|x| x.msg == msg_2));
            assert!(list.data.iter().any(|x| x.msg == msg_3));
        }

        Ok(())
    })
    .await
    .unwrap();

    let list_filtered: ListResponse<EndpointMessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id_1}/msg/?channel=news"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(list_filtered.data.len(), 1);
    assert!(list_filtered.data[0].msg == msg_3);

    // Test 'event_types' query parameter

    let list_balloon_popped: ListResponse<EndpointMessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id_1}/msg/?event_types=balloon.popped",),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(list_balloon_popped.data.len(), 1);
    assert!(list_balloon_popped.data[0].msg == msg_3);

    let list_event_type: ListResponse<EndpointMessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id_1}/msg/?event_types=event.type",),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(list_event_type.data.len(), 2);
    assert!(list_event_type.data.iter().any(|x| x.msg == msg_1));
    assert!(list_event_type.data.iter().any(|x| x.msg == msg_2));

    let list_both_event_types: ListResponse<EndpointMessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id_1}/msg/?event_types=event.type,balloon.popped",),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(list_both_event_types.data.len(), 3);
    assert!(list_both_event_types.data.iter().any(|x| x.msg == msg_1));
    assert!(list_both_event_types.data.iter().any(|x| x.msg == msg_2));
    assert!(list_both_event_types.data.iter().any(|x| x.msg == msg_3));
}

#[tokio::test]
async fn test_list_attempted_messages_failed() {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = vec![Duration::from_millis(1)];
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);
    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let msg_1 = create_test_message(&client, &app_id, json!({ "test": "data1" }))
        .await
        .unwrap();
    let msg_2 = create_test_message(&client, &app_id, json!({ "test": "data2" }))
        .await
        .unwrap();

    run_with_retries(async || {
        for status in ["0"] {
            let list_success: ListResponse<EndpointMessageOut> = client
                .get(
                    &format!("api/v1/app/{app_id}/endpoint/{endp_id}/msg/?status={status}"),
                    StatusCode::OK,
                )
                .await?;

            anyhow::ensure!(list_success.data.len() == 2);
            anyhow::ensure!(list_success.data.iter().any(|x| x.msg == msg_1));
            anyhow::ensure!(list_success.data.iter().any(|x| x.msg == msg_2));
        }
        Ok(())
    })
    .await
    .unwrap();

    receiver.set_response_status_code(StatusCode::INTERNAL_SERVER_ERROR);

    let msg_3 = create_test_message(&client, &app_id, json!({ "test": "data3" }))
        .await
        .unwrap();
    let msg_4 = create_test_message(&client, &app_id, json!({ "test": "data4" }))
        .await
        .unwrap();

    run_with_retries(async || {
        for status in ["2"] {
            let list_failed: ListResponse<EndpointMessageOut> = client
                .get(
                    &format!("api/v1/app/{app_id}/endpoint/{endp_id}/msg/?status={status}"),
                    StatusCode::OK,
                )
                .await?;

            anyhow::ensure!(list_failed.data.len() == 2);
            anyhow::ensure!(list_failed.data.iter().any(|x| x.msg == msg_3));
            anyhow::ensure!(list_failed.data.iter().any(|x| x.msg == msg_4));
        }
        Ok(())
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_list_attempted_messages_sending() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);
    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let msg_1 = create_test_message(&client, &app_id, json!({ "test": "data1" }))
        .await
        .unwrap();
    let msg_2 = create_test_message(&client, &app_id, json!({ "test": "data2" }))
        .await
        .unwrap();

    run_with_retries(async || {
        for status in ["0"] {
            let list_success: ListResponse<EndpointMessageOut> = client
                .get(
                    &format!("api/v1/app/{app_id}/endpoint/{endp_id}/msg/?status={status}"),
                    StatusCode::OK,
                )
                .await?;

            anyhow::ensure!(list_success.data.len() == 2);
            anyhow::ensure!(list_success.data.iter().any(|x| x.msg == msg_1));
            anyhow::ensure!(list_success.data.iter().any(|x| x.msg == msg_2));
        }

        Ok(())
    })
    .await
    .unwrap();

    receiver.set_response_status_code(StatusCode::INTERNAL_SERVER_ERROR);

    let msg_3 = create_test_message(&client, &app_id, json!({ "test": "data3" }))
        .await
        .unwrap();
    let msg_4 = create_test_message(&client, &app_id, json!({ "test": "data4" }))
        .await
        .unwrap();

    run_with_retries(async || {
        for status in ["3"] {
            let list_sending: ListResponse<EndpointMessageOut> = client
                .get(
                    &format!("api/v1/app/{app_id}/endpoint/{endp_id}/msg/?status={status}"),
                    StatusCode::OK,
                )
                .await?;

            anyhow::ensure!(list_sending.data.len() == 2);
            anyhow::ensure!(list_sending.data.iter().any(|x| x.msg == msg_3));
            anyhow::ensure!(list_sending.data.iter().any(|x| x.msg == msg_4));
        }

        Ok(())
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_list_attempted_destinations() {
    use svix_server::v1::endpoints::attempt::MessageEndpointOut;

    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "testListAttemptedDestinations")
        .await
        .unwrap()
        .id;

    let mut receiver_1 =
        TestReceiver::start_with_body(StatusCode::OK, axum::Json(json!({ "ok": true })));
    let endp_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap();

    let mut receiver_2 =
        TestReceiver::start_with_body(StatusCode::OK, axum::Json(json!({ "ok": true })));
    let endp_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
        .await
        .unwrap();

    let mut receiver_3 =
        TestReceiver::start_with_body(StatusCode::OK, axum::Json(json!({ "ok": true })));
    let endp_3 = create_test_endpoint(&client, &app_id, &receiver_3.endpoint)
        .await
        .unwrap();

    let msg_id = create_test_message(&client, &app_id, json!({ "test": "data" }))
        .await
        .unwrap()
        .id;

    receiver_1.recv_body_value().await;
    receiver_2.recv_body_value().await;
    receiver_3.recv_body_value().await;

    let destinations = run_with_retries(async || {
        let list: ListResponse<MessageEndpointOut> = client
            .get(
                &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/"),
                StatusCode::OK,
            )
            .await?;
        anyhow::ensure!(list.data.len() == 3);

        Ok(list)
    })
    .await
    .unwrap();

    let endpoint_ids: Vec<_> = destinations.data.iter().map(|d| d.id.clone()).collect();
    assert_eq!(
        endpoint_ids,
        vec![endp_1.id.clone(), endp_2.id.clone(), endp_3.id.clone()]
    );

    // Delete one endpoint
    client
        .delete(
            &format!("api/v1/app/{app_id}/endpoint/{}/", endp_2.id),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    // Should still see all 3 endpoints (including deleted)
    let destinations_after_delete: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(destinations_after_delete.data.len(), 3);

    let first_page: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/?limit=2"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(first_page.data.len(), 2);
    let endpoint_ids: Vec<_> = first_page.data.iter().map(|d| d.id.clone()).collect();
    assert_eq!(endpoint_ids, vec![endp_1.id.clone(), endp_2.id.clone()]);

    let next_iter = first_page.iterator.unwrap().clone();
    let second_page: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/?limit=2&iterator={next_iter}"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(second_page.data.len(), 1);
    let endpoint_ids: Vec<_> = second_page.data.iter().map(|d| d.id.clone()).collect();
    assert_eq!(endpoint_ids, vec![endp_3.id.clone()]);

    // Test backward pagination
    let all_destinations: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let last_id = &all_destinations.data.last().unwrap().id;

    let prev_page: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/?limit=1&iterator=-{last_id}"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(prev_page.data.len(), 1);
    let endpoint_ids: Vec<_> = prev_page.data.iter().map(|d| &d.id).collect();
    assert_eq!(endpoint_ids, vec![&endp_2.id]);

    let prev_iter = prev_page.prev_iterator.unwrap();
    let prev_page: ListResponse<MessageEndpointOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/{msg_id}/endpoint/?limit=1&iterator={prev_iter}"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(prev_page.data.len(), 1);
    let endpoint_ids: Vec<_> = prev_page.data.iter().map(|d| &d.id).collect();
    assert_eq!(endpoint_ids, vec![&endp_1.id]);

    receiver_1.jh.abort();
    receiver_2.jh.abort();
    receiver_3.jh.abort();
}

#[tokio::test]
async fn test_list_attempts_by_endpoint() {
    let (client, _jh) = start_svix_server().await;

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
    let msg_3 = create_test_msg_with(
        &client,
        &app_id,
        serde_json::json!({"test": "data3"}),
        "user.exploded",
        ["obits"],
    )
    .await;

    // And wait at most one second for all attempts to be processed
    run_with_retries(|| async {
        for endp_id in [endp_id_1.clone(), endp_id_2.clone()] {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id}/"),
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
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_1}/"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    let list_2: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/"),
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

    let foo_attempts: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?channel=foo"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert!(foo_attempts.data.is_empty());

    let obits_attempts: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?channel=obits"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(obits_attempts.data.len(), 1);

    let exploded_attempts: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?event_types=user.exploded"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(exploded_attempts.data.len(), 1);

    let regular_attempts: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?event_types[]=event.type"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(regular_attempts.data.len(), 2);

    let all_attempts_1: ListResponse<MessageAttemptOut> = client
    .get(
        &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?event_types[0]=event.type&event_types[1]=user.exploded"),
        StatusCode::OK,
    )
    .await
    .unwrap();
    assert_eq!(all_attempts_1.data.len(), 3);

    let all_attempts_2: ListResponse<MessageAttemptOut> = client
    .get(
        &format!("api/v1/app/{app_id}/attempt/endpoint/{endp_id_2}/?event_types=event.type,user.exploded"),
        StatusCode::OK,
    )
    .await
    .unwrap();
    assert_eq!(all_attempts_2.data.len(), 3);

    receiver_1.jh.abort();
    receiver_2.jh.abort();
}

#[tokio::test]
async fn test_message_attempts() {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = (0..2).map(|_| Duration::from_millis(1)).collect();

    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

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

        for i in list.data.iter() {
            assert_eq!(i.status, msg_status);
            println!("{} {status_code}", i.response_status_code);
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

    for i in list.data.iter() {
        assert_eq!(i.status, MessageStatus::Fail);
        assert_eq!(i.response_status_code, 0);
        assert_eq!(i.endpoint_id, endp_id);
    }
}

#[tokio::test]
async fn test_message_attempts_empty_retry_schedule() {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = vec![];

    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let (status_code, msg_status, attempt_count) =
        (StatusCode::INTERNAL_SERVER_ERROR, MessageStatus::Fail, None);
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

    for i in list.data.iter() {
        assert_eq!(i.status, msg_status);
        println!("{} {status_code}", i.response_status_code);
        assert_eq!(
            i.response_status_code,
            TryInto::<i16>::try_into(status_code.as_u16()).unwrap()
        );
        assert_eq!(i.endpoint_id, endp_id);
    }
    receiver.jh.abort();
}

#[tokio::test]
async fn test_combined_before_after_filtering() {
    let (client, _) = start_svix_server().await;

    let app = create_test_app(&client, "test_app").await.unwrap();

    let receiver = TestReceiver::start(StatusCode::OK);

    let ep = create_test_endpoint(&client, &app.id, &receiver.endpoint)
        .await
        .unwrap();

    // Send a first message
    create_test_message(
        &client,
        &app.id,
        serde_json::json!({
            "test": 1,
        }),
    )
    .await
    .unwrap();

    // Wait until attempt was made
    run_with_retries(|| async {
        let list: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/endpoint/{}/", app.id, ep.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        if list.data.len() != 1 {
            anyhow::bail!("list len {}, not 1", list.data.len());
        }

        Ok(())
    })
    .await
    .unwrap();

    let ts1 = chrono::Utc::now();

    // Send another two messages
    for i in 1..=2 {
        create_test_message(
            &client,
            &app.id,
            serde_json::json!({
                "test": i + 1,
            }),
        )
        .await
        .unwrap();
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

        if list.data.len() != 3 {
            anyhow::bail!("list len {}, not 3", list.data.len());
        }

        Ok(())
    })
    .await
    .unwrap();
    let ts2 = chrono::Utc::now();

    // Send another three messages
    for i in 1..=3 {
        create_test_message(
            &client,
            &app.id,
            serde_json::json!({
                "test": i + 3,
            }),
        )
        .await
        .unwrap();
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

        if list.data.len() != 6 {
            anyhow::bail!("list len {}, not 6", list.data.len());
        }

        Ok(())
    })
    .await
    .unwrap();

    // No timestamp-based filtering should yield all 6 messages
    let out: ListResponse<MessageAttemptOut> = client
        .get(
            &format!("api/v1/app/{}/attempt/endpoint/{}/?limit=10", app.id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert!(out.done);
    assert_eq!(out.data.len(), 6);

    // Limiting the time to the second batch should only yield those two messages
    let out: ListResponse<MessageAttemptOut> = client
        .get(
            &format!(
                "api/v1/app/{}/attempt/endpoint/{}/\
                 ?limit=10&before={ts2}&after={ts1}",
                app.id, ep.id
            ),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // We got all the data there is for the filters we supplied..
    assert!(out.done);
    assert_eq!(out.data.len(), 2);

    // .. but we can still iterate from here when loosening filters.
    let prev_iter = out.prev_iterator.unwrap();
    let iter = out.iterator.unwrap();

    // Can get the older three messages via pagination
    let out: ListResponse<MessageAttemptOut> = client
        .get(
            &format!(
                "api/v1/app/{}/attempt/endpoint/{}/?iterator={prev_iter}",
                app.id, ep.id
            ),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert!(out.done);
    assert_eq!(out.data.len(), 3);

    // Can get the earlier message via pagination
    let out: ListResponse<MessageAttemptOut> = client
        .get(
            &format!(
                "api/v1/app/{}/attempt/endpoint/{}/?iterator={iter}",
                app.id, ep.id
            ),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert!(out.done);
    assert_eq!(out.data.len(), 1);
}

#[tokio::test]
async fn test_pagination_by_endpoint() {
    let (client, _jh) = start_svix_server().await;

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
            async {
                // the requests that depend on time (ie, `before` and `after`) can flake if too many
                // messages are created too close together.
                // This short sleep aims to separate them a little so we can get clean counts.
                tokio::time::sleep(Duration::from_millis(10)).await;
                create_test_message(
                    &client,
                    &app.id,
                    serde_json::json!({
                        "test": i,
                    }),
                )
                .await
                .unwrap()
            }
            .await,
        );
    }

    // Wait until all attempts were made
    run_with_retries(|| async {
        for endp_id in eps.iter().map(|ep| &ep.id) {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{}/attempt/endpoint/{endp_id}/", app.id),
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
        assert_eq!(first_three_by_time.data.len(), 4);
        assert_eq!(
            &all_attempts.data[0..=3],
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
    let (client, _jh) = start_svix_server().await;

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
    for i in 1..=5usize {
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
    messages.push(
        create_test_msg_with(
            &client,
            &app.id,
            serde_json::json!({"test": "data6"}),
            "balloon.popped",
            ["news"],
        )
        .await,
    );

    // Wait until all attempts were made
    run_with_retries(|| async {
        for endp_id in eps.iter().map(|ep| &ep.id) {
            let list: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!("api/v1/app/{}/attempt/endpoint/{endp_id}/", app.id),
                    StatusCode::OK,
                )
                .await
                .unwrap();

            if list.data.len() != 6 {
                anyhow::bail!("list len {}, not 6", list.data.len());
            }

            let list_filtered: ListResponse<MessageAttemptOut> = client
                .get(
                    &format!(
                        "api/v1/app/{}/attempt/endpoint/{endp_id}/?channel=news",
                        app.id
                    ),
                    StatusCode::OK,
                )
                .await
                .unwrap();
            assert_eq!(list_filtered.data.len(), 1);
        }

        Ok(())
    })
    .await
    .unwrap();

    // By message
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
    fn add_5ms<T: chrono::TimeZone>(ts: chrono::DateTime<T>) -> chrono::DateTime<T> {
        ts + chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
    }

    /// Subtracts 5ms to a [`chrono::DateTime`] for testing `before` and `after`
    fn sub_5ms<T: chrono::TimeZone>(ts: chrono::DateTime<T>) -> chrono::DateTime<T> {
        ts - chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap()
    }
}

#[tokio::test]
async fn test_pagination_forward_and_back() {
    let (client, _) = start_svix_server().await;

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
            format!("&iterator={iter}")
        } else {
            String::new()
        };

        let mut out: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=10{iter_suffix}",
                    app.id, ep.id
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
            format!("&iterator={iter}")
        } else {
            String::new()
        };

        let mut out: ListResponse<MessageAttemptOut> = client
            .get(
                &format!(
                    "api/v1/app/{}/attempt/endpoint/{}/?limit=10{iter_suffix}",
                    app.id, ep.id
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
