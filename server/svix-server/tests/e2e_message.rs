// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::v1::{endpoints::message::MessageOut, utils::ListResponse};

mod utils;

use utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    start_svix_server,
};

#[tokio::test]
async fn test_message_create_read_list() {
    let (client, _jh) = start_svix_server();

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

    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}", &app_id, &message_1.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_1
    );
    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}", &app_id, &message_2.id),
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
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&message_1));
    assert!(list.data.contains(&message_2));
}
