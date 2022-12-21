// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::utils::common_calls::metadata;
use anyhow::Result;
use chrono::Utc;
use ed25519_compact::Signature;
use reqwest::StatusCode;
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryResult, Statement};
use std::sync::Arc;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use serde::Deserialize;
use svix::webhooks::Webhook;
use svix_server::cfg::DefaultSignatureType;
use svix_server::core::types::{BaseId, OrganizationId};
use svix_server::{
    core::{
        cryptography::{AsymmetricKey, Encryption},
        types::{
            ApplicationId, EndpointHeaders, EndpointHeadersPatch, EndpointSecret,
            EndpointSecretInternal, EndpointUid, EventChannel, EventChannelSet, EventTypeName,
            EventTypeNameSet, ExpiringSigningKeys,
        },
    },
    v1::{
        endpoints::{
            endpoint::{
                EndpointHeadersIn, EndpointHeadersOut, EndpointHeadersPatchIn, EndpointIn,
                EndpointOut, EndpointSecretOut, RecoverIn,
            },
            event_type::EventTypeOut,
            message::{MessageIn, MessageOut},
        },
        utils::ListResponse,
    },
};

mod utils;

use utils::{
    common_calls::{
        common_test_list, create_test_app, create_test_endpoint, create_test_message,
        delete_test_app, endpoint_in, event_type_in, get_msg_attempt_list_and_assert_count,
        post_endpoint, put_endpoint, recover_webhooks,
    },
    get_default_test_config, start_svix_server, start_svix_server_with_cfg,
    start_svix_server_with_cfg_and_org_id, IgnoredResponse, TestClient, TestReceiver,
};

async fn get_endpoint(
    client: &TestClient,
    app_id: &ApplicationId,
    ep_id: &str,
) -> Result<EndpointOut> {
    client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
            StatusCode::OK,
        )
        .await
}

async fn get_endpoint_404(
    client: &TestClient,
    app_id: &str,
    ep_id: &str,
) -> Result<IgnoredResponse> {
    client
        .get(
            &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
            StatusCode::NOT_FOUND,
        )
        .await
}

async fn delete_endpoint(client: &TestClient, app_id: &ApplicationId, ep_id: &str) -> Result<()> {
    let _: IgnoredResponse = client
        .delete(
            &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
            StatusCode::NO_CONTENT,
        )
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_patch() {
    let (client, _jh) = start_svix_server().await;

    let app = create_test_app(&client, "v1EndpointPatchTestApp")
        .await
        .unwrap()
        .id;
    let ep = create_test_endpoint(&client, &app, "http://bad.url")
        .await
        .unwrap()
        .id;

    let url = format!("api/v1/app/{app}/endpoint/{ep}/");

    // Test that the description may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "description": "test"
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.description, "test".to_owned());
    // Assert that no other changes were made
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url".to_owned());
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that the rate limit may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "rateLimit": 1,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.rate_limit, Some(1));
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url".to_owned());
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that the rate limit may be unset
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "rateLimit": null,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.rate_limit, None);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url".to_owned());
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that the UID may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "uid": "some",
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.uid, Some(EndpointUid("some".to_owned())));
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.url, "http://bad.url".to_owned());
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test the UID may be unset
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "uid": null,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.uid, None);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.url, "http://bad.url".to_owned());
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that the URL may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "url": "http://bad.url2",
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.version, 1);
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that the version may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "version": 2,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.version, 2);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert!(!out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that disabled may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "disabled": true,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert!(out.ep.disabled);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert_eq!(out.ep.version, 2);
    assert_eq!(out.ep.event_types_ids, None);
    assert_eq!(out.ep.channels, None);

    // Test that event type IDs may be set

    // But first make an event type to set it to
    let _: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            serde_json::json!({
                "description": "a test event type",
                "name": "test",
            }),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "filterTypes": [ "test" ],
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(
        out.ep.event_types_ids,
        Some(EventTypeNameSet(HashSet::from([EventTypeName(
            "test".to_owned()
        )])))
    );
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert_eq!(out.ep.version, 2);
    assert!(out.ep.disabled);
    assert_eq!(out.ep.channels, None);

    // Test that event type IDs may be unset
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "filterTypes": null,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.event_types_ids, None);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert_eq!(out.ep.version, 2);
    assert!(out.ep.disabled);
    assert_eq!(out.ep.channels, None);

    // Test that channels may be set
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "channels": [ "test" ],
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(
        out.ep.channels,
        Some(EventChannelSet(HashSet::from([EventChannel(
            "test".to_owned()
        )])))
    );
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert_eq!(out.ep.version, 2);
    assert!(out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);

    // Test that channels may be unset
    let _: EndpointOut = client
        .patch(
            &url,
            serde_json::json!({
                "channels": null,
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Assert the change was made
    let out = client
        .get::<EndpointOut>(&url, StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(out.ep.channels, None);
    // Assert that no other changes were made
    assert_eq!(out.ep.description, "test".to_owned());
    assert_eq!(out.ep.rate_limit, None);
    assert_eq!(out.ep.uid, None);
    assert_eq!(out.ep.url, "http://bad.url2".to_owned());
    assert_eq!(out.ep.version, 2);
    assert!(out.ep.disabled);
    assert_eq!(out.ep.event_types_ids, None);
}

#[tokio::test]
async fn test_crud() {
    let (client, _jh) = start_svix_server().await;

    const APP_NAME_1: &str = "v1EndpointCrudTestApp1";
    const APP_NAME_2: &str = "v1EndpointCrudTestApp2";

    const EP_URI_APP_1_EP_1_VER_1: &str = "http://v1EndpointCrudTestApp1Ep1Ver1.test";
    const EP_URI_APP_1_EP_1_VER_2: &str = "http://v1EndpointCrudTestApp1Ep1Ver2.test";
    const EP_URI_APP_1_EP_2: &str = "http://v1EndpointCrudTestApp1Ep2.test";
    const EP_URI_APP_2_EP_1: &str = "http://v1EndpointCrudTestApp2Ep1.test";
    const EP_URI_APP_2_EP_2: &str = "http://v1EndpointCrudTestApp2Ep2.test";

    let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap().id;
    let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap().id;

    // CREATE
    let app_1_ep_1 = create_test_endpoint(&client, &app_1, EP_URI_APP_1_EP_1_VER_1)
        .await
        .unwrap();
    assert_eq!(app_1_ep_1.ep.url, EP_URI_APP_1_EP_1_VER_1);
    assert_eq!(app_1_ep_1.ep.version, 1);

    let app_1_ep_2 = create_test_endpoint(&client, &app_1, EP_URI_APP_1_EP_2)
        .await
        .unwrap();
    assert_eq!(app_1_ep_2.ep.url, EP_URI_APP_1_EP_2);
    assert_eq!(app_1_ep_2.ep.version, 1);

    let app_2_ep_1 = create_test_endpoint(&client, &app_2, EP_URI_APP_2_EP_1)
        .await
        .unwrap();
    assert_eq!(app_2_ep_1.ep.url, EP_URI_APP_2_EP_1);
    assert_eq!(app_2_ep_1.ep.version, 1);

    let app_2_ep_2 = create_test_endpoint(&client, &app_2, EP_URI_APP_2_EP_2)
        .await
        .unwrap();
    assert_eq!(app_2_ep_2.ep.url, EP_URI_APP_2_EP_2);
    assert_eq!(app_2_ep_2.ep.version, 1);

    // READ

    // Can read from correct app
    assert_eq!(
        get_endpoint(&client, &app_1, &app_1_ep_1.id).await.unwrap(),
        app_1_ep_1
    );
    assert_eq!(
        get_endpoint(&client, &app_1, &app_1_ep_2.id).await.unwrap(),
        app_1_ep_2
    );
    assert_eq!(
        get_endpoint(&client, &app_2, &app_2_ep_1.id).await.unwrap(),
        app_2_ep_1
    );
    assert_eq!(
        get_endpoint(&client, &app_2, &app_2_ep_2.id).await.unwrap(),
        app_2_ep_2
    );

    // Can't read from incorrect app
    get_endpoint_404(&client, &app_2, &app_1_ep_1.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_2, &app_1_ep_2.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_1, &app_2_ep_1.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_1, &app_2_ep_2.id)
        .await
        .unwrap();

    // UPDATE
    let app_1_ep_1_id = app_1_ep_1.id;
    let app_1_ep_1: EndpointOut = client
        .put(
            &format!("api/v1/app/{app_1}/endpoint/{app_1_ep_1_id}/"),
            endpoint_in(EP_URI_APP_1_EP_1_VER_2),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(app_1_ep_1.ep.url, EP_URI_APP_1_EP_1_VER_2);

    // CONFIRM UPDATE
    assert_eq!(
        get_endpoint(&client, &app_1, &app_1_ep_1_id).await.unwrap(),
        app_1_ep_1
    );

    // Test that PUT with an invalid ID creates an endpoint
    let app_1_ep_3: EndpointOut = client
        .put(
            &format!("api/v1/app/{app_1}/endpoint/fake-id/"),
            endpoint_in(EP_URI_APP_1_EP_1_VER_2),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // LIST
    let list_app_1: ListResponse<EndpointOut> = client
        .get(&format!("api/v1/app/{}/endpoint/", &app_1), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list_app_1.data.len(), 3);
    assert!(list_app_1.data.contains(&app_1_ep_1));
    assert!(list_app_1.data.contains(&app_1_ep_2));
    assert!(list_app_1.data.contains(&app_1_ep_3));

    let list_app_2: ListResponse<EndpointOut> = client
        .get(&format!("api/v1/app/{}/endpoint/", &app_2), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list_app_2.data.len(), 2);
    assert!(list_app_2.data.contains(&app_2_ep_1));
    assert!(list_app_2.data.contains(&app_2_ep_2));

    // DELETE
    delete_endpoint(&client, &app_1, &app_1_ep_1.id)
        .await
        .unwrap();
    delete_endpoint(&client, &app_1, &app_1_ep_2.id)
        .await
        .unwrap();
    delete_endpoint(&client, &app_2, &app_2_ep_1.id)
        .await
        .unwrap();
    delete_endpoint(&client, &app_2, &app_2_ep_2.id)
        .await
        .unwrap();

    // CONFIRM DELETION
    get_endpoint_404(&client, &app_1, &app_1_ep_1.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_1, &app_1_ep_2.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_2, &app_2_ep_1.id)
        .await
        .unwrap();
    get_endpoint_404(&client, &app_2, &app_2_ep_2.id)
        .await
        .unwrap();

    let mut ep_with_metadata = endpoint_in("https://somewhere.beyond.the.c");
    ep_with_metadata.metadata = metadata(r#"{"foo": "bar", "bizz": "baz"}"#);
    let ep = post_endpoint(&client, &app_1, ep_with_metadata)
        .await
        .unwrap();
    assert_eq!(ep.metadata, metadata(r#"{"foo": "bar", "bizz": "baz"}"#));

    let ep_alias = get_endpoint(&client, &app_1, &ep.id).await.unwrap();
    assert_eq!(
        ep_alias.metadata,
        metadata(r#"{"foo": "bar", "bizz": "baz"}"#)
    );

    // Test that metadata may be unset
    let ep_alias2: EndpointOut = client
        .patch(
            &format!("api/v1/app/{}/endpoint/{}/", &app_1, &ep.id),
            serde_json::json!({
                "metadata": {},
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(ep_alias2.metadata, metadata(r#"{}"#));
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "App1").await.unwrap().id;
    common_test_list::<EndpointOut, EndpointIn>(
        &client,
        &format!("api/v1/app/{app_id}/endpoint/"),
        |i| endpoint_in(&format!("https://localhost/{i}")),
        true,
    )
    .await
    .unwrap();
}

/// Tests that there is at most one endpoint with a single UID for all endpoints associated with
/// any application
#[tokio::test]
async fn test_uid() {
    let (client, _jh) = start_svix_server().await;

    const APP_NAME_1: &str = "v1EndpointUidTestApp1";
    const APP_NAME_2: &str = "v1EndpointUidTestApp2";

    const EP_URI_APP_1_EP_1: &str = "http://v1EndpointUidTestApp1Ep1.test";
    const EP_URI_APP_1_EP_2: &str = "http://v1EndpointUidTestApp1Ep2.test";
    const EP_URI_APP_2: &str = "http://v1EndpointUidTestApp2Ep1.test";

    const DUPLICATE_UID: &str = "test_uid";

    // Same App

    // Double Create -- on creation, it should return an error if identical UIDs are used for
    // endpoints in the same app
    let app_id = create_test_app(&client, APP_NAME_1).await.unwrap().id;
    let uid = EndpointUid(DUPLICATE_UID.to_owned());

    let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
    ep_1.uid = Some(uid.clone());

    let mut ep_2 = endpoint_in(EP_URI_APP_1_EP_2);
    ep_2.uid = Some(uid.clone());

    let ep_1 = post_endpoint(&client, &app_id, ep_1).await.unwrap();

    client
        .post::<_, IgnoredResponse>(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_2,
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    // Update One to Existing -- on update it should return an error if attempting to change
    // the UID to that of an existing endpoint associated with the same app
    let ep_2 = create_test_endpoint(&client, &app_id, EP_URI_APP_1_EP_2)
        .await
        .unwrap();

    let mut ep_2_with_duplicate_uid = endpoint_in(EP_URI_APP_1_EP_2);
    ep_2_with_duplicate_uid.uid = Some(uid.clone());

    client
        .put::<_, IgnoredResponse>(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_2.id),
            ep_2_with_duplicate_uid,
            StatusCode::CONFLICT,
        )
        .await
        .unwrap();

    // Update One to Identical -- however it should not return an error if updating the
    // existing endpoint to one with the same UID
    let mut ep_1_with_duplicate_id = endpoint_in(EP_URI_APP_1_EP_1);
    ep_1_with_duplicate_id.uid = Some(uid.clone());

    let ep_1_updated = client
        .put::<_, EndpointOut>(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_1.id),
            ep_1_with_duplicate_id,
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(ep_1.id, ep_1_updated.id);
    assert_eq!(ep_1.ep.uid, ep_1_updated.ep.uid);

    // Delete One then Create One -- UIDs may be reused after deletion
    delete_endpoint(&client, &app_id, &ep_1.id).await.unwrap();
    delete_endpoint(&client, &app_id, &ep_2.id).await.unwrap();

    let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
    ep_1.uid = Some(uid.clone());
    client
        .post::<_, IgnoredResponse>(
            &format!("api/v1/app/{}/endpoint/", &app_id),
            ep_1,
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    delete_test_app(&client, app_id).await.unwrap();

    // Different App -- however if they are associated with different applications, identical
    // UIDs are valid
    let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap().id;
    let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap().id;

    let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
    ep_1.uid = Some(uid.clone());

    let mut ep_2 = endpoint_in(EP_URI_APP_2);
    ep_2.uid = Some(uid.clone());

    let _ = post_endpoint(&client, &app_1, ep_1).await.unwrap();
    let _ = post_endpoint(&client, &app_2, ep_2).await.unwrap();
}

// Simply tests that upon rotating an endpoint secret that it differs from the prior one
#[tokio::test]
async fn test_endpoint_secret_get_and_rotation() {
    let (client, _jh) = start_svix_server().await;

    const APP_NAME: &str = "v1EndpointSecretRotationTestApp";
    const EP_URI: &str = "http://v1EndpointSecretRotationTestEp.test";

    let app_id = create_test_app(&client, APP_NAME).await.unwrap().id;

    let ep = create_test_endpoint(&client, &app_id, EP_URI)
        .await
        .unwrap();

    let former_secret: EndpointSecretOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
            serde_json::json!({ "key": null }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    assert_ne!(
        former_secret,
        client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                StatusCode::OK
            )
            .await
            .unwrap()
    );

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
            &former_secret,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    assert_eq!(
        former_secret,
        client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                StatusCode::OK
            )
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn test_recovery_should_fail_if_start_time_too_old() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::INTERNAL_SERVER_ERROR);

    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let _: serde_json::Value = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id}/recover/"),
            RecoverIn {
                since: Utc::now() - chrono::Duration::weeks(3),
            },
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_recovery_expected_retry_counts() {
    let mut cfg = get_default_test_config();

    cfg.retry_schedule = (0..2)
        .into_iter()
        .map(|_| Duration::from_millis(1))
        .collect();

    // total attempts for a failed message should be 1 (first attempt) + length of retry_schedule:
    let base_attempt_cnt = 1 + &cfg.retry_schedule.len();

    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::INTERNAL_SERVER_ERROR);

    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let before_msg = Utc::now();

    let msg = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();

    get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, base_attempt_cnt)
        .await
        .unwrap();

    let after_msg = Utc::now();

    // recovery time after msg -- should be no additional attempts
    recover_webhooks(
        &client,
        after_msg,
        &format!("api/v1/app/{app_id}/endpoint/{endp_id}/recover/"),
    )
    .await;

    get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, base_attempt_cnt)
        .await
        .unwrap();

    // recovery time before msg -- should be 1 additional attempt
    recover_webhooks(
        &client,
        before_msg,
        &format!("api/v1/app/{app_id}/endpoint/{endp_id}/recover/"),
    )
    .await;

    get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, base_attempt_cnt + 1)
        .await
        .unwrap();

    receiver.jh.abort();
}

#[tokio::test]
async fn test_endpoint_rotate_max() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let endp_id = create_test_endpoint(&client, &app_id, "http://www.example.com")
        .await
        .unwrap()
        .id;

    for _ in 0..ExpiringSigningKeys::MAX_OLD_KEYS {
        let _: IgnoredResponse = client
            .post(
                &format!("api/v1/app/{app_id}/endpoint/{endp_id}/secret/rotate/"),
                serde_json::json!({ "key": null }),
                StatusCode::NO_CONTENT,
            )
            .await
            .unwrap();
    }

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id}/secret/rotate/"),
            serde_json::json!({ "key": null }),
            StatusCode::BAD_REQUEST,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_endpoint_rotate_signing_e2e() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let endp = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap();

    let secret1: EndpointSecretOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, endp.id),
            serde_json::json!({ "key": null }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let secret2: EndpointSecretOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_ne!(secret1.key, secret2.key);

    let secret3_key = EndpointSecretInternal::generate_symmetric(&Encryption::new_noop())
        .unwrap()
        .into_endpoint_secret(&Encryption::new_noop())
        .unwrap();

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, endp.id),
            serde_json::json!({ "key": secret3_key }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let secret3: EndpointSecretOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(secret3_key, secret3.key);

    let raw_payload = r#"{"test":"data1"}"#;
    let payload = serde_json::from_str(raw_payload).unwrap();
    let _msg = create_test_message(&client, &app_id, payload)
        .await
        .unwrap();

    let last_headers = receiver.header_recv.recv().await.unwrap();
    let last_body = receiver.data_recv.recv().await.unwrap().to_string();

    for sec in [secret1, secret2, secret3] {
        if let EndpointSecret::Symmetric(key) = &sec.key {
            let sec = base64::encode(key);
            let wh = Webhook::new(sec).unwrap();
            wh.verify(last_body.as_bytes(), &last_headers).unwrap();
        } else {
            panic!("Shouldn't get here");
        }
    }
}

#[tokio::test]
async fn test_endpoint_rotate_signing_symmetric_and_asymmetric() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let secret_1 = EndpointSecretInternal::generate_symmetric(&Encryption::new_noop())
        .unwrap()
        .into_endpoint_secret(&Encryption::new_noop())
        .unwrap();
    // Asymmetric key
    let secret_2 = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==").unwrap());
    // Long key
    let secret_3 = EndpointSecret::Symmetric(base64::decode("TUdfVE5UMnZlci1TeWxOYXQtX1ZlTW1kLTRtMFdhYmEwanIxdHJvenRCbmlTQ2hFdzBnbHhFbWdFaTJLdzQwSA==").unwrap());

    let ep_in = EndpointIn {
        url: receiver.endpoint.clone(),
        version: 1,
        key: Some(secret_1.clone()),
        ..Default::default()
    };

    let endp = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    // Rotate to asmmetric
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, endp.id),
            serde_json::json!({ "key": "whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==" }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    // Rotate back to symmetric
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, endp.id),
            serde_json::json!({ "key": secret_3.serialize_public_key() }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let raw_payload = r#"{"test":"data1"}"#;
    let payload = serde_json::from_str(raw_payload).unwrap();
    let _msg = create_test_message(&client, &app_id, payload)
        .await
        .unwrap();

    let last_headers = receiver.header_recv.recv().await.unwrap();
    let last_body = receiver.data_recv.recv().await.unwrap().to_string();

    for sec in [secret_1, secret_2, secret_3] {
        match sec {
            EndpointSecret::Symmetric(key) => {
                let sec = base64::encode(key);
                let wh = Webhook::new(sec).unwrap();
                wh.verify(last_body.as_bytes(), &last_headers).unwrap();
            }
            EndpointSecret::Asymmetric(key) => {
                let msg_id = last_headers.get("svix-id").unwrap().to_str().unwrap();
                let timestamp = last_headers
                    .get("svix-timestamp")
                    .unwrap()
                    .to_str()
                    .unwrap();
                let signatures = last_headers
                    .get("svix-signature")
                    .unwrap()
                    .to_str()
                    .unwrap();
                let to_sign = format!("{}.{}.{}", msg_id, timestamp, &last_body);
                let found =
                    signatures
                        .split(' ')
                        .filter(|x| x.starts_with("v1a,"))
                        .any(|signature| {
                            let sig: Signature = Signature::from_slice(
                                base64::decode(&signature["v1a,".len()..])
                                    .unwrap()
                                    .as_slice(),
                            )
                            .unwrap();
                            key.0.pk.verify(to_sign.as_bytes(), &sig).is_ok()
                        });
                assert!(found);
            }
        }
    }
}

#[tokio::test]
async fn test_endpoint_secret_config() {
    let mut cfg = get_default_test_config();
    cfg.default_signature_type = DefaultSignatureType::Ed25519;
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        ..Default::default()
    };

    let ep = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    #[derive(Deserialize)]
    pub struct EndpointSecretOutTest {
        pub key: String,
    }

    let key1 = client
        .get::<EndpointSecretOutTest>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap()
        .key;

    assert!(key1.starts_with("whpk_"));

    // Rotate to asmmetric
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
            serde_json::json!({ "key": null }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let key2 = client
        .get::<EndpointSecretOutTest>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap()
        .key;

    assert!(key2.starts_with("whpk_"));
}

#[tokio::test]
async fn test_custom_endpoint_secret() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let secret_1 = EndpointSecretInternal::generate_symmetric(&Encryption::new_noop())
        .unwrap()
        .into_endpoint_secret(&Encryption::new_noop())
        .unwrap();
    // Long key
    let secret_2 = EndpointSecret::Symmetric(base64::decode("TUdfVE5UMnZlci1TeWxOYXQtX1ZlTW1kLTRtMFdhYmEwanIxdHJvenRCbmlTQ2hFdzBnbHhFbWdFaTJLdzQwSA==").unwrap());
    // Asymmetric key
    let secret_3 = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==").unwrap());
    assert_eq!(
        secret_3.serialize_public_key(),
        "whpk_1SiA4o9hyqTCpIqC5V9HUakiiaeACeqfZTInDBbOir4="
    );

    let mut ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        key: Some(secret_1.clone()),
        ..Default::default()
    };

    let endp_1 = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    ep_in.key = Some(secret_2.clone());
    let endp_2 = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    // We rotate the key after because it's easier than setting json! for everything
    ep_in.key = Some(secret_2.clone());
    let endp_3 = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, endp_3.id),
            serde_json::json!({ "key": "whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==" }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    #[derive(Deserialize)]
    pub struct EndpointSecretOutTest {
        pub key: String,
    }

    for (secret, ep) in [(secret_1, endp_1), (secret_2, endp_2), (secret_3, endp_3)] {
        assert_eq!(
            secret.serialize_public_key(),
            client
                .get::<EndpointSecretOutTest>(
                    &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                    StatusCode::OK
                )
                .await
                .unwrap()
                .key
        );
    }
}

#[tokio::test]
async fn test_endpoint_secret_encryption() {
    let org_id = OrganizationId::new(None, None);
    let cfg = get_default_test_config();
    let (client, _jh) = start_svix_server_with_cfg_and_org_id(&cfg, org_id.clone()).await;

    #[derive(Deserialize)]
    pub struct EndpointSecretOutTest {
        pub key: String,
    }

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        ..Default::default()
    };

    let ep = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    let secret = client
        .get::<EndpointSecretOutTest>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap()
        .key;

    // Now add encryption and check the secret is still fine
    let mut cfg = get_default_test_config();
    cfg.encryption = Encryption::new([1; 32]);
    let (client, _jh) = start_svix_server_with_cfg_and_org_id(&cfg, org_id.clone()).await;

    let secret2 = client
        .get::<EndpointSecretOutTest>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap()
        .key;

    // Ensure loading the existing secret works
    assert_eq!(secret, secret2);

    // Generate a new encrypted secret
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
            serde_json::json!({ "key": secret }),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let secret2 = client
        .get::<EndpointSecretOutTest>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::OK,
        )
        .await
        .unwrap()
        .key;

    // Ensure loading and saving works for encrypted
    assert_eq!(secret, secret2);

    // Make sure we can't read it with the secret unset
    let cfg = get_default_test_config();
    let (client, _jh) = start_svix_server_with_cfg_and_org_id(&cfg, org_id.clone()).await;
    client
        .get::<IgnoredResponse>(
            &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_invalid_endpoint_secret() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let secret_too_short = "whsec_C2FVsBQIhrscChlQIM+b5sSYspob".to_owned();
    let secret_too_long =
        "whsec_V09IYXZUaFJoSnFobnpJQkpPMXdpdGFNWnJsRzAxdXZCeTVndVpwRmxSSXFsc0oyYzBTRWRUekJhYnlaZ0JSRGNPQ3BGZG1xYjFVVmRGQ3UK"
            .to_owned();
    let invalid_prefix = "hwsec_C2FVsBQIhrscChlQIM+b5sSYspob7oDazfgh".to_owned();

    for sec in [secret_too_short, secret_too_long, invalid_prefix] {
        let ep_in: serde_json::Value = serde_json::json!({
            "url": "http://www.example.com".to_owned(),
            "version": 1,
            "secret": sec,
        });

        let _: IgnoredResponse = client
            .post(
                &format!("api/v1/app/{app_id}/endpoint/"),
                ep_in,
                StatusCode::UNPROCESSABLE_ENTITY,
            )
            .await
            .unwrap();
    }
}

/// We used to store the secret in the DB without a type marker, check loading those still works
#[tokio::test]
async fn test_legacy_endpoint_secret() {
    let cfg = get_default_test_config();
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let db = Arc::new(cfg);
    let db = svix_server::db::init_db(&db).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let secret_throwaway = EndpointSecretInternal::generate_symmetric(&Encryption::new_noop())
        .unwrap()
        .into_endpoint_secret(&Encryption::new_noop())
        .unwrap();
    let raw_key = base64::decode("5gasBsSw3Nvf3ugNYVJIqnRVYPW7hPts").unwrap();
    let secret_1 = EndpointSecret::Symmetric(raw_key.clone());

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        key: Some(secret_throwaway.clone()),
        ..Default::default()
    };

    let endp_1 = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    // Set the raw value to the database (like legacy)
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "UPDATE endpoint SET key = $1 WHERE id = $2",
        vec![raw_key.clone().into(), endp_1.id.clone().into()],
    ))
    .await
    .unwrap();

    let endp_1 = get_endpoint(&client, &app_id, &endp_1.id).await.unwrap();

    #[derive(Deserialize)]
    pub struct EndpointSecretOutTest {
        pub key: String,
    }

    let (secret, ep) = (secret_1, endp_1);
    assert_eq!(
        secret.serialize_public_key(),
        client
            .get::<EndpointSecretOutTest>(
                &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                StatusCode::OK
            )
            .await
            .unwrap()
            .key
    );
}

#[tokio::test]
async fn test_endpoint_secret_encryption_in_database() {
    let mut cfg = get_default_test_config();
    cfg.encryption = Encryption::new([1; 32]);
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let db = Arc::new(cfg);
    let db = svix_server::db::init_db(&db).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        ..Default::default()
    };

    let ep = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    let secret_encrypted: Option<QueryResult> = db
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT key FROM endpoint WHERE id = $1",
            vec![ep.id.clone().into()],
        ))
        .await
        .unwrap();
    let secret_encrypted: Vec<u8> = secret_encrypted.unwrap().try_get("", "key").unwrap();

    let cfg = get_default_test_config();
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        ..Default::default()
    };

    let ep = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    let secret_clear: Option<QueryResult> = db
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT key FROM endpoint WHERE id = $1",
            vec![ep.id.clone().into()],
        ))
        .await
        .unwrap();
    let secret_clear: Vec<u8> = secret_clear.unwrap().try_get("", "key").unwrap();

    // Ensure that the length of the encrypted is much longer than the clear
    assert!(secret_encrypted.len() > secret_clear.len() + 10);
}

#[tokio::test]
async fn test_endpoint_filter_events() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_empty_events: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1,
        "filterTypes": []
    });

    let ep_with_events: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1,
        "filterTypes": ["et1"]
    });

    let ep_no_events: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1
    });

    let expected_et = EventTypeNameSet(HashSet::from([EventTypeName("et1".to_owned())]));

    let _ep_with_empty_events: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_empty_events,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

    let _ep_with_nonexistent_event: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_with_events.to_owned(),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

    let _et: EventTypeOut = client
        .post(
            "api/v1/event-type/",
            event_type_in("et1", None).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let ep_with_valid_event: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_with_events.to_owned(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(ep_with_valid_event.ep.event_types_ids.unwrap(), expected_et);

    let ep_removed_events: EndpointOut = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_with_valid_event.id),
            ep_no_events.to_owned(),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert!(ep_removed_events.ep.event_types_ids.is_none());

    let ep_removed_events = get_endpoint(&client, &app_id, &ep_removed_events.id)
        .await
        .unwrap();

    assert!(ep_removed_events.ep.event_types_ids.is_none());

    let ep_updated_events: EndpointOut = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_with_valid_event.id),
            ep_with_events.to_owned(),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(ep_updated_events.ep.event_types_ids.unwrap(), expected_et);

    let ep_updated_events: EndpointOut = get_endpoint(&client, &app_id, &ep_with_valid_event.id)
        .await
        .unwrap();

    assert_eq!(ep_updated_events.ep.event_types_ids.unwrap(), expected_et);
}

#[tokio::test]
async fn test_endpoint_filter_channels() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    // Channels must not be empty:
    let ep_empty_channels: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1,
        "channels": []
    });

    let ep_with_channels: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1,
        "channels": ["tag1"]
    });

    let ep_without_channels: serde_json::Value = serde_json::json!({
        "url": "http://www.example.com",
        "version": 1
    });

    let expected_ec = EventChannelSet(HashSet::from([EventChannel("tag1".to_owned())]));

    let _ep_w_empty_channel: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_empty_channels,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

    let ep_with_channel: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep_with_channels.to_owned(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    assert_eq!(ep_with_channel.ep.channels.unwrap(), expected_ec);

    let ep_with_deleted_channel: EndpointOut = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_with_channel.id),
            ep_without_channels,
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert!(ep_with_deleted_channel.ep.channels.is_none());

    // GET / assert channels empty
    let ep_with_deleted_channel: EndpointOut = get_endpoint(&client, &app_id, &ep_with_channel.id)
        .await
        .unwrap();

    assert!(ep_with_deleted_channel.ep.channels.is_none());

    // Update with channels:
    let updated_ep_with_channel: EndpointOut = client
        .put(
            &format!(
                "api/v1/app/{}/endpoint/{}/",
                app_id, ep_with_deleted_channel.id
            ),
            ep_with_channels,
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(updated_ep_with_channel.ep.channels.unwrap(), expected_ec);

    // GET / assert channels match
    let updated_ep_with_channel: EndpointOut =
        get_endpoint(&client, &app_id, &updated_ep_with_channel.id)
            .await
            .unwrap();

    assert_eq!(updated_ep_with_channel.ep.channels.unwrap(), expected_ec);
}

#[tokio::test]
async fn test_rate_limit() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let ep_in = EndpointIn {
        url: "http://www.example.com".to_owned(),
        version: 1,
        rate_limit: Some(100),
        ..Default::default()
    };

    let endp = post_endpoint(&client, &app_id, ep_in.clone())
        .await
        .unwrap();

    assert_eq!(endp.ep.rate_limit.unwrap(), 100);

    let endp = put_endpoint(
        &client,
        &app_id,
        &endp.id,
        EndpointIn {
            rate_limit: None,
            ..ep_in.clone()
        },
    )
    .await
    .unwrap();

    assert!(endp.ep.rate_limit.is_none());

    let endp = get_endpoint(&client, &app_id, &endp.id).await.unwrap();

    assert!(endp.ep.rate_limit.is_none());
}

#[tokio::test]
async fn test_msg_event_types_filter() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);

    for et in [
        event_type_in("et1", None).unwrap(),
        event_type_in("et2", None).unwrap(),
    ] {
        let _: EventTypeOut = client
            .post("api/v1/event-type/", et, StatusCode::CREATED)
            .await
            .unwrap();
    }

    for event_types in [
        Some(EventTypeNameSet(HashSet::from([EventTypeName(
            "et1".to_owned(),
        )]))),
        Some(EventTypeNameSet(HashSet::from([
            EventTypeName("et1".to_owned()),
            EventTypeName("et2".to_owned()),
        ]))),
        None,
    ] {
        post_endpoint(
            &client,
            &app_id,
            EndpointIn {
                url: receiver.endpoint.to_owned(),
                version: 1,
                event_types_ids: event_types,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }

    // Number of attempts should match based on event-types registered to endpoints
    for (event_name, expected_count) in [
        (EventTypeName("et1".to_owned()), 3),
        (EventTypeName("et2".to_owned()), 2),
    ] {
        let msg: MessageOut = client
            .post(
                &format!("api/v1/app/{}/msg/", &app_id),
                MessageIn {
                    channels: None,
                    event_type: event_name,
                    payload: serde_json::json!({}),
                    uid: None,
                    payload_retention_period: 5,
                },
                StatusCode::ACCEPTED,
            )
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(10)).await;

        let _list =
            get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, expected_count)
                .await
                .unwrap();
    }
}

#[tokio::test]
async fn test_msg_channels_filter() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);

    let ec = EventChannelSet(HashSet::from([EventChannel("tag1".to_owned())]));

    for channels in [Some(ec.clone()), None] {
        let _endp = post_endpoint(
            &client,
            &app_id,
            EndpointIn {
                url: receiver.endpoint.to_owned(),
                version: 1,
                channels,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }

    for (channels, expected_count) in [(Some(ec.clone()), 2), (None, 1)] {
        let msg: MessageOut = client
            .post(
                &format!("api/v1/app/{}/msg/", &app_id),
                MessageIn {
                    channels: channels.clone(),
                    event_type: EventTypeName("et1".to_owned()),
                    payload: serde_json::json!({}),
                    uid: None,
                    payload_retention_period: 5,
                },
                StatusCode::ACCEPTED,
            )
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let _list =
            get_msg_attempt_list_and_assert_count(&client, &app_id, &msg.id, expected_count)
                .await
                .unwrap();

        let msg: MessageOut = client
            .get(
                &format!("api/v1/app/{}/msg/{}/", &app_id, &msg.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(msg.channels, channels);
    }
}

#[tokio::test]
async fn test_endpoint_headers_manipulation() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let endp = create_test_endpoint(&client, &app_id, "http://www.example.com")
        .await
        .unwrap();

    let patched_headers_in = EndpointHeadersPatchIn {
        headers: EndpointHeadersPatch(HashMap::from([
            ("x-test-3".to_owned(), Some("4".to_owned())),
            ("x-test-2".to_owned(), None),
        ])),
    };

    let _: IgnoredResponse = client
        .patch(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            patched_headers_in,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let recvd_headers: EndpointHeadersOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(
        HashMap::from([("x-test-3".to_owned(), "4".to_owned()),]),
        recvd_headers.headers
    );

    let endp = create_test_endpoint(&client, &app_id, "http://www.example.com")
        .await
        .unwrap();

    for bad_hdr in [
        "content-length",
        "some:thing",
        "some\u{0000}thing",
        "svix-foo",
        "x-svix-foo",
        "x-amzn-foo",
    ] {
        let _: IgnoredResponse = client
            .put(
                &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
                serde_json::json!({ "headers": { bad_hdr: "123"}}),
                StatusCode::UNPROCESSABLE_ENTITY,
            )
            .await
            .unwrap();
    }

    let org_headers = EndpointHeadersIn {
        headers: EndpointHeaders(HashMap::from([
            ("x-test-1".to_owned(), "1".to_owned()),
            ("x-test-2".to_owned(), "2".to_owned()),
        ])),
    };

    let updated_headers = EndpointHeadersIn {
        headers: EndpointHeaders(HashMap::from([
            ("x-test-1".to_owned(), "3".to_owned()),
            ("x-test-2".to_owned(), "2".to_owned()),
        ])),
    };

    for hdrs in [&org_headers, &updated_headers] {
        let _: IgnoredResponse = client
            .put(
                &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
                hdrs,
                StatusCode::NO_CONTENT,
            )
            .await
            .unwrap();

        let recvd_headers: EndpointHeadersOut = client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert_eq!(hdrs.headers.0, recvd_headers.headers);
    }

    let patched_headers_in = EndpointHeadersPatchIn {
        headers: EndpointHeadersPatch(HashMap::from([
            ("x-test-3".to_owned(), Some("4".to_owned())),
            ("x-test-2".to_owned(), None),
        ])),
    };

    let _: IgnoredResponse = client
        .patch(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            &patched_headers_in,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let recvd_headers: EndpointHeadersOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(
        HashMap::from([
            ("x-test-1".to_owned(), "3".to_owned()),
            ("x-test-3".to_owned(), "4".to_owned()),
        ]),
        recvd_headers.headers
    );

    let redacted_headers = EndpointHeadersIn {
        headers: EndpointHeaders(HashMap::from([
            ("x-test-1".to_owned(), "1".to_owned()),
            ("authorization".to_owned(), "secret".to_owned()),
        ])),
    };

    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            redacted_headers,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let recvd_headers: EndpointHeadersOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(
        HashMap::from([("x-test-1".to_owned(), "1".to_owned())]),
        recvd_headers.headers
    );

    assert_eq!(
        HashSet::from(["authorization".to_owned()]),
        recvd_headers.sensitive
    );
}

#[tokio::test]
async fn test_endpoint_headers_sending() {
    let (client, _jh) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let endp = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap();

    let headers = EndpointHeadersIn {
        headers: EndpointHeaders(HashMap::from([
            ("x-test-1".to_owned(), "1".to_owned()),
            ("x-test-2".to_owned(), "2".to_owned()),
        ])),
    };

    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            &headers,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();

    let last_headers = receiver.header_recv.recv().await.unwrap();

    for (k, v) in &headers.headers.0 {
        assert_eq!(v, last_headers.get(k).unwrap().to_str().unwrap());
    }
}

#[tokio::test]
async fn test_endpoint_header_key_capitalization() {
    let (client, _jk) = start_svix_server().await;

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::OK);

    let endp = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap();

    let headers = EndpointHeadersIn {
        headers: EndpointHeaders(HashMap::from([(
            "X-Api-Test".to_owned(),
            "test-value".to_owned(),
        )])),
    };

    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            &headers,
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let retrieved_headers: EndpointHeadersOut = client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/headers/", app_id, endp.id),
            StatusCode::OK,
        )
        .await
        .unwrap();

    for k in headers.headers.0.keys() {
        assert!(retrieved_headers.headers.contains_key(k));
    }
}

/// Tests the endpoint_https_only configuration
#[tokio::test]
async fn test_endpoint_https_only() {
    let http_url = "http://www.example.com";
    let https_url = "https://www.example.com";

    // No https enforcement (default)
    let cfg = get_default_test_config();
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "App 1").await.unwrap().id;

    assert!(!cfg.endpoint_https_only);

    let _endpoint: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            endpoint_in(https_url),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _endpoint: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            endpoint_in(http_url),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Enforce https
    let mut cfg = get_default_test_config();
    cfg.endpoint_https_only = true;
    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "App 1").await.unwrap().id;

    assert!(cfg.endpoint_https_only);

    let _endpoint: EndpointOut = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            endpoint_in(https_url),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            endpoint_in(http_url),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}
