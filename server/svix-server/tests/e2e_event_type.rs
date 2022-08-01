// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::v1::{
    endpoints::event_type::{EventTypeIn, EventTypeOut},
    utils::ListResponse,
};

mod utils;

use utils::{
    common_calls::{common_test_list, event_type_in},
    start_svix_server,
};

#[tokio::test]
async fn test_patch() {
    let (client, _jh) = start_svix_server();

    let et: EventTypeOut = client
        .post(
            "api/v1/event-type",
            event_type_in("test-event-type", serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Test that description may be set while the rest are omitted
    let _: EventTypeOut = client
        .patch(
            &format!("api/v1/event-type/{}/", et.name),
            serde_json::json!({
                "description": "updated_description",
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert that the change was made
    let out = client
        .get::<EventTypeOut>(&format!("api/v1/event-type/{}/", &et.name), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.description, "updated_description".to_owned());

    // Assert the other fields remain unchanged
    assert_eq!(out.deleted, et.deleted);
    assert_eq!(out.schemas, et.schemas);

    // Test that schemas may be set while the rest are omitted
    let _: EventTypeOut = client
        .patch(
            &format!("api/v1/event-type/{}/", et.name),
            serde_json::json!({
                "schemas": {},
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert that the change was made
    let out = client
        .get::<EventTypeOut>(&format!("api/v1/event-type/{}/", &et.name), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(out.schemas, Some(serde_json::json!({})));

    // Assert the other fields remain unchanged
    assert_eq!(out.deleted, et.deleted);
    assert_eq!(out.description, "updated_description".to_owned());

    // Test that schemas may be unset while the rest are omitted
    let _: EventTypeOut = client
        .patch(
            &format!("api/v1/event-type/{}/", et.name),
            serde_json::json!({
                "schemas": null,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert that the change was made
    let out = client
        .get::<EventTypeOut>(&format!("api/v1/event-type/{}/", &et.name), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(out.schemas, None);

    // Assert the other fields remain unchanged
    assert_eq!(out.deleted, et.deleted);
    assert_eq!(out.description, "updated_description".to_owned());

    // Test that deleted may be set while the rest are omitted
    let _: EventTypeOut = client
        .patch(
            &format!("api/v1/event-type/{}/", et.name),
            serde_json::json!({
                "archived": true,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert that the change was made
    let out = client
        .get::<EventTypeOut>(&format!("api/v1/event-type/{}/", &et.name), StatusCode::OK)
        .await
        .unwrap();

    assert!(out.deleted);

    // Assert the other fields remain unchanged
    assert_eq!(out.schemas, None);
    assert_eq!(out.description, "updated_description".to_owned());
}

#[tokio::test]
async fn test_event_type_create_read_list() {
    let (client, _jh) = start_svix_server();

    let et: EventTypeOut = client
        .post(
            "api/v1/event-type",
            event_type_in("test-event-type", serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(
        client
            .get::<EventTypeOut>(&format!("api/v1/event-type/{}/", &et.name), StatusCode::OK)
            .await
            .unwrap(),
        et
    );

    let list: ListResponse<EventTypeOut> = client
        .get("api/v1/event-type/?with_content=true", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(list.data.contains(&et));

    let list: ListResponse<EventTypeOut> = client
        .get("api/v1/event-type", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(list.data.contains(&EventTypeOut {
        schemas: None,
        ..et
    }));
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server();

    common_test_list::<EventTypeOut, EventTypeIn>(
        &client,
        "api/v1/event-type/",
        |i| {
            event_type_in(
                &format!("test-event-type-{i}"),
                serde_json::json!({"test": "value"}),
            )
            .unwrap()
        },
        true,
    )
    .await
    .unwrap();
}
