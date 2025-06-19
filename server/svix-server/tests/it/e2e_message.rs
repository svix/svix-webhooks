// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::{Duration, Utc};
use reqwest::StatusCode;
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter};
use serde::de::IgnoredAny;
use serde_json::json;
use svix_server::{
    db::models::messagecontent,
    expired_message_cleaner,
    v1::{
        endpoints::{
            attempt::MessageAttemptOut,
            message::{MessageOut, RawPayload},
        },
        utils::ListResponse,
    },
};

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_msg_with, message_in},
    run_with_retries, start_svix_server, TestReceiver,
};

#[tokio::test]
async fn test_message_create_read_list() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    // CREATE
    let message_1: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, json!({ "test": "value" })).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let message_2: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, json!({ "test": "value2" })).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let message_3 = create_test_msg_with(
        &client,
        &app_id,
        json!({ "test": "data3" }),
        "balloon.popped",
        ["news"],
    )
    .await;

    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{app_id}/msg/{}/", message_1.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_1
    );
    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{app_id}/msg/{}/", message_2.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_2
    );

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{app_id}/msg/"), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 3);
    assert!(list.data.contains(&message_1));
    assert!(list.data.contains(&message_2));
    assert!(list.data.contains(&message_3));

    let list: ListResponse<MessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/?channel=news"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(list.data.contains(&message_3));
}

#[tokio::test]
async fn test_message_create_read_list_with_content() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    let msg_payload = json!({ "test": "value" });

    let msg_1_w_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(
        msg_1_w_payload.payload.0.get(),
        serde_json::to_string(&msg_payload).unwrap()
    );

    let msg_2_wo_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/?with_content=false"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg_2_wo_payload.payload.0.get(), "{}");

    let msg_1_wo_payload = MessageOut {
        payload: RawPayload::from_string("{}".to_string()).unwrap(),
        ..msg_1_w_payload.clone()
    };
    let msg_2_w_payload = MessageOut {
        payload: RawPayload::from_string(serde_json::to_string(&msg_payload).unwrap()).unwrap(),
        ..msg_2_wo_payload.clone()
    };

    for m in [&msg_1_w_payload, &msg_2_w_payload] {
        assert_eq!(
            client
                .get::<MessageOut>(
                    &format!("api/v1/app/{app_id}/msg/{}/", m.id),
                    StatusCode::OK
                )
                .await
                .unwrap(),
            *m,
        );
    }

    for m in [&msg_1_wo_payload, &msg_2_wo_payload] {
        assert_eq!(
            client
                .get::<MessageOut>(
                    &format!("api/v1/app/{app_id}/msg/{}/?with_content=false", m.id),
                    StatusCode::OK
                )
                .await
                .unwrap(),
            *m
        );
    }

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{app_id}/msg/"), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&msg_1_w_payload));
    assert!(list.data.contains(&msg_2_w_payload));

    let list: ListResponse<MessageOut> = client
        .get(
            &format!("api/v1/app/{app_id}/msg/?with_content=false"),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&msg_1_wo_payload));
    assert!(list.data.contains(&msg_2_wo_payload));
}

#[tokio::test]
async fn test_failed_message_gets_recorded() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let mut receiver = TestReceiver::start(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let _endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let msg_payload = json!({ "test": "value" });

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    receiver.data_recv.recv().await;

    run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/msg/{}/", msg_res.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        if !attempts.data.iter().any(|x| x.response_status_code == 500) {
            anyhow::bail!("could not find failed attempt");
        }

        Ok(())
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_multiple_endpoints() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let mut receiver_1 = TestReceiver::start(axum::http::StatusCode::OK);
    let mut receiver_2 = TestReceiver::start(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    let mut receiver_3 = TestReceiver::start(axum::http::StatusCode::OK);

    let _endp_id_1 = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap()
        .id;
    let _endp_id_2 = create_test_endpoint(&client, &app_id, &receiver_2.endpoint)
        .await
        .unwrap()
        .id;
    let _endp_id_3 = create_test_endpoint(&client, &app_id, &receiver_3.endpoint)
        .await
        .unwrap()
        .id;

    let msg_payload = json!({ "test": "value1" });

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let (rec_body_1, rec_body_2, rec_body_3) = tokio::join!(
        receiver_1.data_recv.recv(),
        receiver_2.data_recv.recv(),
        receiver_3.data_recv.recv()
    );

    assert_eq!(msg_payload.to_string(), rec_body_1.unwrap().to_string());
    assert_eq!(msg_payload.to_string(), rec_body_2.unwrap().to_string());
    assert_eq!(msg_payload.to_string(), rec_body_3.unwrap().to_string());

    receiver_2.set_response_status_code(axum::http::StatusCode::OK);

    let rec_body_2 = receiver_2.data_recv.recv().await.unwrap();

    assert_eq!(msg_payload.to_string(), rec_body_2.to_string());

    run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/msg/{}/", msg_res.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        if !attempts.data.iter().any(|x| x.response_status_code == 200) {
            anyhow::bail!("could not find successful attempt");
        }

        Ok(())
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_failed_message_gets_requeued() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let mut receiver_1 = TestReceiver::start(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    let _endp_id = create_test_endpoint(&client, &app_id, &receiver_1.endpoint)
        .await
        .unwrap()
        .id;

    let msg_payload = json!({ "test": "value" });

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    receiver_1.data_recv.recv().await;

    receiver_1.set_response_status_code(axum::http::StatusCode::OK);

    let last_body = receiver_1.data_recv.recv().await.unwrap();

    assert_eq!(msg_payload.to_string(), last_body.to_string());

    run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/msg/{}/", msg_res.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        if !attempts.data.iter().any(|x| x.response_status_code == 200) {
            anyhow::bail!("could not find successful attempt");
        }

        Ok(())
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_payload_retention_period() {
    let (client, _jh) = start_svix_server().await;
    dotenvy::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");
    let pool = svix_server::db::init_db(&cfg).await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let msg: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, json!({ "test": "value" })).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let msg_id = msg.id.clone();

    let content: Option<messagecontent::Model> = messagecontent::Entity::find_by_id(msg_id.clone())
        .one(&pool)
        .await
        .unwrap();
    assert_eq!(content.unwrap().id, msg_id.clone());

    let res = messagecontent::Entity::update_many()
        .col_expr(
            messagecontent::Column::Expiration,
            Expr::value(Utc::now() - Duration::days(1)),
        )
        .filter(messagecontent::Column::Id.eq(msg_id.clone()))
        .exec(&pool)
        .await
        .unwrap();
    assert_eq!(1, res.rows_affected);

    expired_message_cleaner::clean_expired_messages(&pool, 5000, false)
        .await
        .unwrap();

    let content: Option<messagecontent::Model> = messagecontent::Entity::find_by_id(msg_id)
        .one(&pool)
        .await
        .unwrap();
    assert!(content.is_none());
}

#[tokio::test]
async fn test_payload_retention_period_messagecontent() {
    let (client, _jh) = start_svix_server().await;
    dotenvy::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");
    let pool = svix_server::db::init_db(&cfg).await;

    let app_id = create_test_app(&client, "test-content-expiration-period")
        .await
        .unwrap()
        .id;

    let custom_retention_period = 5;
    let msg: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            json!({
                "eventType": "test.event",
                "payload": { "test": "value" },
                "payloadRetentionPeriod": custom_retention_period
            }),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let msg_id = msg.id.clone();

    let content: messagecontent::Model = messagecontent::Entity::find_by_id(msg_id.clone())
        .one(&pool)
        .await
        .unwrap()
        .unwrap();

    let expected = Utc::now() + Duration::days(custom_retention_period) + Duration::hours(1);
    let actual: chrono::DateTime<Utc> = content.expiration.into();

    assert!(actual < expected);
}

#[tokio::test]
async fn test_expunge_message_payload() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "testApp").await.unwrap().id;

    let payload = json!({ "sensitive": "data" });
    let msg: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(
        msg.payload.0.get(),
        serde_json::to_string(&payload).unwrap()
    );

    let msg = client
        .get::<MessageOut>(
            &format!("api/v1/app/{app_id}/msg/{}/", msg.id),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(
        msg.payload.0.get(),
        serde_json::to_string(&payload).unwrap()
    );

    client
        .delete(
            &format!("api/v1/app/{app_id}/msg/{}/content/", msg.id),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let msg = client
        .get::<MessageOut>(
            &format!("api/v1/app/{app_id}/msg/{}/", msg.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(msg.payload.0.get(), r#"{"expired":true}"#);
}

#[tokio::test]
async fn test_message_conflict() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    let msg_in = json!({
        "eventType": "user.signup",
        "payload": { "test": "value" },
        "payloadRetentionPeriod": 5,
        "eventId": "test1",
    });

    let _: MessageOut = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            msg_in.clone(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let _: IgnoredAny = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            msg_in,
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_message_validation() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "testApp").await.unwrap().id;
    let payload = json!({ "large_payload": "payload-".repeat(1_000_000) });

    client
        .post::<_, IgnoredAny>(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, payload).unwrap(),
            StatusCode::PAYLOAD_TOO_LARGE,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_raw_payload() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "testRawPayload").await.unwrap().id;

    let mut receiver = TestReceiver::start(axum::http::StatusCode::OK);

    create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap();

    let msg_payload = json!({ "test": "value1" });

    let _: IgnoredAny = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            json!({
                "eventType": "payload.raw",
                "payload": {},
                "transformationsParams": {
                    "rawPayload": msg_payload.to_string(),
                },
            }),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let rec_body = receiver.data_recv.recv().await;
    assert_eq!(msg_payload.to_string(), rec_body.unwrap().to_string());
}
