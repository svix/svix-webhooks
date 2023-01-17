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
    common_calls::{application_in, common_test_list, dashboard_access, event_type_in},
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
                name: EventTypeName("foo-event".to_owned()),
                description: "test-event-description".to_owned(),
                deleted: false,
                schemas: None,
                feature_flag: Some(feature),
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

    // Client with no feature flags set
    let client1 = dashboard_access(&client, &app, FeatureFlagSet::default()).await;
    // Client with a different set of feature flags than needed
    let client2 = dashboard_access(&client, &app, other_features).await;
    // Client with the right flag, plus extras
    let client3 = dashboard_access(&client, &app, union.clone()).await;
    // Client with only the right flag
    let client4 = dashboard_access(&client, &app, features.clone()).await;

    // Clients which don't have the right flag shouldn't see it
    let list: ListResponse<EventTypeOut> = client1
        .get("api/v1/event-type/", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 0);
    let path = format!("api/v1/event-type/{}/", et.name);
    let _: IgnoredResponse = client1.get(&path, StatusCode::NOT_FOUND).await.unwrap();

    let list: ListResponse<EventTypeOut> = client2
        .get("api/v1/event-type/", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 0);
    let path = format!("api/v1/event-type/{}/", et.name);
    let _: IgnoredResponse = client2.get(&path, StatusCode::NOT_FOUND).await.unwrap();

    // Clients with the right flags set should see the event type
    let list: ListResponse<EventTypeOut> = client3
        .get("api/v1/event-type/", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(list.data.contains(&et));
    let got_et: EventTypeOut = client3.get(&path, StatusCode::OK).await.unwrap();
    assert_eq!(et, got_et);

    let list: ListResponse<EventTypeOut> = client4
        .get("api/v1/event-type/", StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(list.data.contains(&et));
    let got_et: EventTypeOut = client3.get(&path, StatusCode::OK).await.unwrap();
    assert_eq!(et, got_et);
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
