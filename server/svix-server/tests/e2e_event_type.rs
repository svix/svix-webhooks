// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use reqwest::StatusCode;

use svix_server::{
    core::types::{ApplicationId, EventTypeName, FeatureFlag, FeatureFlagSet},
    db::models::eventtype::Schema,
    v1::{
        endpoints::{
            application::ApplicationOut,
            event_type::{EventTypeIn, EventTypeOut},
        },
        utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{app_portal_access, application_in, common_test_list, event_type_in},
    start_svix_server, IgnoredResponse,
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
async fn test_event_type_feature_flags() {
    let (client, _jh) = start_svix_server().await;

    let feature = FeatureFlag("foo-feature".into());
    let another_feature = FeatureFlag("bar-feature".into());
    let (features, other_features, union) = {
        let mut s1 = HashSet::new();
        s1.insert(feature.clone());
        let mut s2 = HashSet::new();
        s2.insert(another_feature);
        let union: FeatureFlagSet = s1.union(&s2).cloned().collect();

        (s1, s2, union)
    };

    let et: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            EventTypeIn {
                name: EventTypeName("event-type-with-flag".to_owned()),
                description: "test-event-description".to_owned(),
                deleted: false,
                schemas: None,
                feature_flag: Some(feature),
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            EventTypeIn {
                name: EventTypeName("no-flag-event".to_owned()),
                description: "test-event-description".to_owned(),
                deleted: false,
                schemas: None,
                feature_flag: None,
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let app: ApplicationId = client
        .post::<_, ApplicationOut>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;

    let path = format!("api/v1/event-type/{}/", et.name);

    for (flag_set, should_see) in [
        (FeatureFlagSet::default(), false),
        (other_features, false),
        (union.clone(), true),
        (features.clone(), true),
    ] {
        let client = app_portal_access(&client, &app, flag_set).await;

        let list: ListResponse<EventTypeOut> = client
            .get("api/v1/event-type/", StatusCode::OK)
            .await
            .unwrap();

        if should_see {
            // If the client is expected to see both event types it should be able to retrieve it
            let got_et: EventTypeOut = client.get(&path, StatusCode::OK).await.unwrap();
            assert_eq!(et, got_et);

            // ... and see it in the list.
            assert_eq!(list.data.len(), 2);
            assert!(list.data.contains(&et));
        } else {
            // If the client is not supposed to see it it shouldn't be able to retrieve it
            let _: IgnoredResponse = client.get(&path, StatusCode::NOT_FOUND).await.unwrap();

            // ... and it shouldn't be in the list.
            assert_eq!(list.data.len(), 1);
            assert!(!list.data.contains(&et));
        };
    }
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server().await;

    common_test_list::<EventTypeOut, EventTypeIn>(
        &client,
        "api/v1/event-type/",
        |i| event_type_in(&format!("test-event-type-{i}"), None).unwrap(),
        true,
        false,
    )
    .await
    .unwrap();
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
