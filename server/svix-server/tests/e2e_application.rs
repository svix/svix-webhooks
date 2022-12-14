// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::utils::common_calls::metadata;
use reqwest::StatusCode;
use svix_server::{
    cfg::CacheType, core::types::ApplicationUid, v1::endpoints::application::ApplicationIn,
    v1::endpoints::application::ApplicationOut,
};

mod utils;

use utils::{
    common_calls::{application_in, common_test_list},
    start_svix_server, IgnoredResponse,
};

// NOTE: PATCHing must be tested exhaustively as if any of the boilerplate is missed then the
// operation could fail. This should probably be made into a macro if at all possible.
#[tokio::test]
async fn test_patch() {
    let (client, _jh) = start_svix_server().await;

    let app: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in("first_name"),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Test that PUT with an invalid ID creates an application
    let _: ApplicationOut = client
        .put(
            "api/v1/app/fake-id/",
            application_in("first_name"),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Test that name may be set while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json! ({
                "name": "second_name"
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert change was made when later fetched
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.name, "second_name".to_owned());
    // Assert that no other field was changed
    assert_eq!(out.rate_limit, None);
    assert_eq!(out.uid, None);
    assert_eq!(out.metadata, metadata("{}"));

    // Test that rate_limit may be set while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json! ({
                "rateLimit": 1,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.rate_limit, Some(1));
    // Assert that no other field was changed
    assert_eq!(out.name, "second_name".to_owned());
    assert_eq!(out.uid, None);

    // Test that rate_limit may be unset while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json!({ "rateLimit": null }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.rate_limit, None);
    // Assert that no other field was changed
    assert_eq!(out.name, "second_name".to_owned());
    assert_eq!(out.uid, None);

    // Test that uid may be set while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json! ({
                "uid": "test_uid"
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.uid, Some(ApplicationUid("test_uid".to_owned())));
    // Assert that no other field was changed
    assert_eq!(out.name, "second_name".to_owned());
    assert_eq!(out.rate_limit, None);

    // Test that uid may be unset while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json!({ "uid": null }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.uid, None);
    // Assert that no other field was changed
    assert_eq!(out.name, "second_name".to_owned());
    assert_eq!(out.rate_limit, None);

    // Test that metadata may be changed while the rest are omitted
    let _: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json!({
                "metadata": {
                    "foo": "bar",
                    "bizz": "baz",
                },
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<ApplicationOut>(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(metadata(r#"{"foo": "bar", "bizz": "baz"}"#), out.metadata);
    // Assert that no other field was changed
    assert_eq!(out.name, "second_name".to_owned());
    assert_eq!(out.rate_limit, None);
}

#[tokio::test]
async fn test_crud() {
    let (client, _jh) = start_svix_server().await;

    const APP_NAME_1_1: &str = "v1ApplicationCrudTest11";
    const APP_NAME_1_2: &str = "v1ApplicationCrudTest12";
    const APP_NAME_2_1: &str = "v1ApplicationCrudTest21";
    const APP_NAME_2_2: &str = "v1ApplicationCrudTest22";

    // CREATE
    let app_1: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_1_1),
            StatusCode::CREATED,
        )
        .await
        .unwrap();
    assert_eq!(app_1.name, APP_NAME_1_1);

    let app_2: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_2_1),
            StatusCode::CREATED,
        )
        .await
        .unwrap();
    assert_eq!(app_2.name, APP_NAME_2_1);

    // READ
    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_1.id), StatusCode::OK)
            .await
            .unwrap(),
        app_1
    );

    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_2.id), StatusCode::OK,)
            .await
            .unwrap(),
        app_2
    );

    // UPDATE
    let app_1_id = app_1.id;
    let app_1: ApplicationOut = client
        .put(
            &format!("api/v1/app/{app_1_id}/"),
            application_in(APP_NAME_1_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let app_2_id = app_2.id;
    let app_2: ApplicationOut = client
        .put(
            &format!("api/v1/app/{app_2_id}/"),
            application_in(APP_NAME_2_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // CONFIRM UPDATE
    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{app_1_id}/"), StatusCode::OK,)
            .await
            .unwrap(),
        app_1
    );

    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{app_2_id}/"), StatusCode::OK,)
            .await
            .unwrap(),
        app_2
    );

    // DELETE
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{}/", app_1.id), StatusCode::NO_CONTENT)
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{}/", app_2.id), StatusCode::NO_CONTENT)
        .await
        .unwrap();

    // CONFIRM DELETION
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{}/", app_1.id), StatusCode::NOT_FOUND)
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{}/", app_2.id), StatusCode::NOT_FOUND)
        .await
        .unwrap();

    let app: ApplicationOut = client
        .post(
            "api/v1/app/",
            serde_json::json!({
                "name": "Apps all around",
            }),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(app.metadata, metadata(r#"{}"#));

    let updated: ApplicationOut = client
        .patch(
            &format!("api/v1/app/{}/", app.id),
            serde_json::json!({
                "metadata": {
                    "bizz": "bar"
                },
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(updated.metadata, metadata(r#"{"bizz":"bar"}"#));

    let new_app: ApplicationOut = client
        .put(
            "api/v1/app/one_upserted_boi/",
            serde_json::json!({
                "name": "Apps for two",
                "metadata": {
                    "foo": "bar"
                },
            }),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(new_app.metadata, metadata(r#"{"foo":"bar"}"#));

    let updated_metadata_app: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}/", new_app.id),
            serde_json::json!({
                "name": "New Name",
                "metadata": {
                    "new": "data"
                },
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(updated_metadata_app.metadata, metadata(r#"{"new":"data"}"#));
    assert_eq!(updated_metadata_app.name, "New Name");
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server().await;

    common_test_list::<ApplicationOut, ApplicationIn>(
        &client,
        "api/v1/app/",
        |i| application_in(&format!("App {i}")),
        true,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_uid() {
    let (client, _jh) = start_svix_server().await;

    let app: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_ne!(app.id.0, app.uid.unwrap().0);

    // Can't create another app with the same uid twice
    let _: IgnoredResponse = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    // Can't update an app to an existing uid (when we have no uid)
    let app2: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 2".to_owned(),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/", app2.id),
            ApplicationIn {
                name: "App 2".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    // Can't update an app to an existing uid (when we have a uid)
    let app2: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 2".to_owned(),
                uid: Some(ApplicationUid("app2".to_owned())),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/", app2.id),
            ApplicationIn {
                name: "App 2".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    // Delete app1
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{}/", app.id), StatusCode::NO_CONTENT)
        .await
        .unwrap();

    // Update to a now deleted uid
    let app2: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}/", app2.id),
            ApplicationIn {
                name: "App 2".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .delete(
            &format!("api/v1/app/{}/", app2.uid.unwrap()),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    // Create an app with the same UID again (after it was deleted)
    let app: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Can update an app with a UID
    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/", app.id),
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Can update the UID
    let app: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}/", app.id),
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app3".to_owned())),
                ..Default::default()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let app2: ApplicationOut = client
        .get(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(app.id, app2.id);
    assert_eq!(app.uid, app2.uid);
    assert_eq!(app2.uid.unwrap().0, "app3");

    // Remove the uid
    let app: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}/", app.id),
            ApplicationIn {
                name: "App 1".to_owned(),
                ..Default::default()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let app2: ApplicationOut = client
        .get(&format!("api/v1/app/{}/", app.id), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(app.id, app2.id);
    assert!(app2.uid.is_none());

    // Make sure we can't fetch by the old UID
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{}/", "app3"), StatusCode::NOT_FOUND)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_uid_across_users() {
    let (client, _jh) = start_svix_server().await;
    let (client2, _jh2) = start_svix_server().await;

    // Make sure that uids aren't unique across different users

    let _app: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _app2: ApplicationOut = client2
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_or_create() {
    let (client, _jh) = start_svix_server().await;

    let app: ApplicationOut = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                metadata: metadata(
                    r#"{
                    "foo": "bar"
                }"#,
                ),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(
        app.metadata,
        metadata(
            r#"{
        "foo": "bar"
    }"#,
        )
    );

    let _: IgnoredResponse = client
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "App 1".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    let app2: ApplicationOut = client
        .post(
            "api/v1/app/?get_if_exists=true",
            ApplicationIn {
                name: "App 1 - SLIGHTLY DIFFERENT, BUT WON'T BE PERSISTED".to_owned(),
                uid: Some(ApplicationUid("app1".to_owned())),
                ..Default::default()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(app, app2);
}

#[tokio::test]
async fn test_idempotency() {
    let (client, _jh) = start_svix_server().await;

    let cfg = svix_server::cfg::load().unwrap();

    let app1: ApplicationOut = client
        .post_with_idempotency(
            "api/v1/app/",
            "1",
            ApplicationIn {
                name: "Some unique name".to_owned(),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let app2: ApplicationOut = client
        .post_with_idempotency(
            "api/v1/app/",
            "1",
            ApplicationIn {
                name: "Some less unique name".to_owned(),
                ..Default::default()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    match cfg.cache_type {
        CacheType::None => assert_ne!(app1, app2),
        _ => assert_eq!(app1, app2),
    };
}
