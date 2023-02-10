// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::{Duration, Utc};
use reqwest::StatusCode;
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter};

use svix_server::{
    db::models::message,
    expired_message_cleaner,
    v1::{
        endpoints::attempt::MessageAttemptOut, endpoints::message::MessageOut, utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_msg_with, message_in},
    run_with_retries, start_svix_server, IgnoredResponse, TestReceiver,
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
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let message_2: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value2"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let message_3 = create_test_msg_with(
        &client,
        &app_id,
        serde_json::json!({"test": "data3"}),
        "balloon.popped",
        ["news"],
    )
    .await;

    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}/", &app_id, &message_1.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_1
    );
    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}/", &app_id, &message_2.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_2
    );

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{}/msg/", &app_id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 3);
    assert!(list.data.contains(&message_1));
    assert!(list.data.contains(&message_2));
    assert!(list.data.contains(&message_3));

    let list: ListResponse<MessageOut> = client
        .get(
            &format!("api/v1/app/{}/msg/?channel=news", &app_id),
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

    let msg_payload = serde_json::json!({"test": "value"});

    let msg_1_w_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg_1_w_payload.payload, msg_payload);

    let msg_2_wo_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/?with_content=false", &app_id),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg_2_wo_payload.payload, serde_json::json!({}));

    let msg_1_wo_payload = MessageOut {
        payload: serde_json::json!({}),
        ..msg_1_w_payload.clone()
    };
    let msg_2_w_payload = MessageOut {
        payload: msg_payload,
        ..msg_2_wo_payload.clone()
    };

    for m in [&msg_1_w_payload, &msg_2_w_payload] {
        assert_eq!(
            client
                .get::<MessageOut>(
                    &format!("api/v1/app/{}/msg/{}/", &app_id, &m.id),
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
                    &format!("api/v1/app/{}/msg/{}/?with_content=false", &app_id, &m.id),
                    StatusCode::OK
                )
                .await
                .unwrap(),
            *m
        );
    }

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{}/msg/", &app_id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&msg_1_w_payload));
    assert!(list.data.contains(&msg_2_w_payload));

    let list: ListResponse<MessageOut> = client
        .get(
            &format!("api/v1/app/{}/msg/?with_content=false", &app_id),
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

    let msg_payload = serde_json::json!({"test": "value"});

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    receiver.data_recv.recv().await;

    run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{}/attempt/msg/{}/", app_id, msg_res.id),
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
async fn test_mulitple_endpoints() {
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

    let msg_payload = serde_json::json!({"test": "value1"});

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
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
                &format!("api/v1/app/{}/attempt/msg/{}/", app_id, msg_res.id),
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

    let msg_payload = serde_json::json!({"test": "value"});

    let msg_res: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
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
                &format!("api/v1/app/{}/attempt/msg/{}/", app_id, msg_res.id),
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
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");
    let pool = svix_server::db::init_db(&cfg).await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let msg_row: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let msg_row_2 = msg_row.clone();

    message::Entity::update_many()
        .col_expr(
            message::Column::Expiration,
            Expr::value(Utc::now() - Duration::days(1)),
        )
        .filter(message::Column::Id.eq(msg_row.id))
        .exec(&pool)
        .await
        .unwrap();

    expired_message_cleaner::clean_expired_messages(&pool)
        .await
        .unwrap();

    let message: Option<message::Model> = message::Entity::find_by_id(msg_row_2.id)
        .one(&pool)
        .await
        .unwrap();

    assert_eq!(message.unwrap().payload, None);
}

#[tokio::test]
async fn test_expunge_message_payload() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "testApp").await.unwrap().id;

    let payload = serde_json::json!({"sensitive": "data"});
    let msg: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg.payload, payload);

    let msg = client
        .get::<MessageOut>(
            &format!("api/v1/app/{}/msg/{}/", &app_id, &msg.id),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(msg.payload, payload);

    let _: IgnoredResponse = client
        .delete(
            &format!("api/v1/app/{}/msg/{}/content/", &app_id, &msg.id),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let msg = client
        .get::<MessageOut>(
            &format!("api/v1/app/{}/msg/{}/", &app_id, &msg.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(msg.payload, serde_json::json!({"expired": true}));
}

#[tokio::test]
async fn test_insert_event_type_into_payload() {
    let (client, _jh) = start_svix_server().await;
    let app_id = create_test_app(&client, "testApp").await.unwrap().id;
    let mut receiver = TestReceiver::start(axum::http::StatusCode::OK);

    let _endp = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let payload = serde_json::json!({"hello": "world"});

    // Default is to not touch the payload
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in("foo", payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let data = receiver.data_recv.recv().await.unwrap();
    assert_eq!(payload, data);

    // If flag is set, insert event type into payload
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/msg/?insert_event_type=true", &app_id),
            message_in("foo", payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let data = receiver.data_recv.recv().await.unwrap();
    assert_eq!(serde_json::json!({"hello":"world", "event":"foo"}), data);

    // If the payload already contains a field called "event" it should not be overwritten
    let payload = serde_json::json!({"event": "bar"});

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/msg/?insert_event_type=true", &app_id),
            message_in("foo", payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let data = receiver.data_recv.recv().await.unwrap();
    assert_eq!(payload, data);
}
