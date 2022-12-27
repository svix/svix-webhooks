// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::{
    db::models::eventtype::Schema,
    v1::{
        endpoints::endpoint::EndpointOut,
        endpoints::event_type::{EventTypeIn, EventTypeOut, RetryScheduleInOut},
        endpoints::message::MessageOut,
        utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{
        common_test_list, create_test_app, event_type_in, get_msg_attempt_list_and_assert_count,
        message_in,
    },
    start_svix_server,
};

#[tokio::test]
async fn test_patch() {
    let (client, _jh) = start_svix_server().await;

    let et: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            event_type_in(
                "test-event-type",
                serde_json::json!({
                    "1": {
                        "type": "object",
                        "title": "Longitude and Latitude Values",
                        "description": "A geographical coordinate.",
                        "required": ["latitude", "longitude"],
                        "properties": {
                        "latitude": {"type": "number", "minimum": -90, "maximum": 90},
                        "longitude": {"type": "number", "minimum": -180, "maximum": 180},
                        },
                    }
                }),
            )
            .unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Test that PUT with invalid ID creates an event type
    let _: EventTypeOut = client
        .put(
            "api/v1/event-type/fake-id/",
            event_type_in("test-event-type", None).unwrap(),
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

    assert_eq!(out.schemas, Some(Schema::default()));

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
    let (client, _jh) = start_svix_server().await;

    let et: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            event_type_in("test-event-type", None).unwrap(),
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
        .get("api/v1/event-type/", StatusCode::OK)
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
    let (client, _jh) = start_svix_server().await;

    common_test_list::<EventTypeOut, EventTypeIn>(
        &client,
        "api/v1/event-type/",
        |i| event_type_in(&format!("test-event-type-{i}"), None).unwrap(),
        true,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_retry_schedule_crud() {
    let (client, _jh) = start_svix_server().await;

    let event_type_name = "test-event-type-for-retry-schedule-override";

    let _: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            event_type_in(event_type_name, None).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let retry_schedule_payload = serde_json::json!({ "retrySchedule": vec![1, 2] });

    let rsi: RetryScheduleInOut = client
        .put(
            &format!("api/v1/event-type/{event_type_name}/retry-schedule/"),
            retry_schedule_payload,
            StatusCode::OK,
        )
        .await
        .unwrap();

    let rso: RetryScheduleInOut = client
        .get(
            &format!("api/v1/event-type/{event_type_name}/retry-schedule/"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(rsi, rso);
}

#[tokio::test]
async fn test_retry_schedule_override() {
    let (client, _jh) = start_svix_server().await;

    let event_type_name = "retry-schedule-override";

    let _: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            event_type_in(event_type_name, None).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: RetryScheduleInOut = client
        .put(
            &format!("api/v1/event-type/{event_type_name}/retry-schedule/"),
            serde_json::json!({ "retrySchedule": vec![1, 1, 1] }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let app_id = create_test_app(&client, "app").await.unwrap().id;

    let bad_url = "http://localhost:4920/bad-url".to_string();

    let _: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            serde_json::json!({
                "url": bad_url.clone(),
                "version": 1,
                "filterTypes": vec![event_type_name]
            }),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let msg: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(event_type_name, serde_json::json!({"test": "data"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let list = get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, 4)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 4);
}

#[tokio::test]
async fn test_schema() {
    let (client, _jh) = start_svix_server().await;
    let _: serde_json::Value = client
        .post(
            "api/v1/event-type/",
            serde_json::json!({
                            "name": "bad-schema",
                            "description": "I have a bad schema",
                            "schemas": {
                                "1": {"readOnly": 15},
                            },
            }),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}
