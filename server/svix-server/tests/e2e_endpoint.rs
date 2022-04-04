// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;

use svix_server::{
    core::types::{ApplicationId, EndpointId, EndpointUid},
    v1::{
        endpoints::{
            endpoint::{EndpointIn, EndpointOut, EndpointSecretOut, RecoverIn},
            message::MessageOut,
        },
        utils::ListResponse,
    },
};

mod utils;

use tokio::time::sleep;
use utils::{
    common_calls::{
        common_test_list, create_test_app, create_test_endpoint, create_test_message,
        delete_test_app, endpoint_in, get_msg_attempt_list_and_assert_count, post_endpoint,
        recover_webhooks, wait_for_msg_retries,
    },
    get_default_test_config, start_svix_server, start_svix_server_with_cfg, IgnoredResponse,
    TestClient, TestReceiver,
};

async fn get_endpoint(
    client: &TestClient,
    app_id: &ApplicationId,
    ep_id: &str,
) -> Result<EndpointOut> {
    client
        .get(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
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
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
            StatusCode::NOT_FOUND,
        )
        .await
}

async fn delete_endpoint(client: &TestClient, app_id: &ApplicationId, ep_id: &str) -> Result<()> {
    let _: IgnoredResponse = client
        .delete(
            &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
            StatusCode::NO_CONTENT,
        )
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_crud() {
    let (client, _jh) = start_svix_server();

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
    assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_1);
    assert_eq!(app_1_ep_1.version, 1);

    let app_1_ep_2 = create_test_endpoint(&client, &app_1, EP_URI_APP_1_EP_2)
        .await
        .unwrap();
    assert_eq!(app_1_ep_2.url, EP_URI_APP_1_EP_2);
    assert_eq!(app_1_ep_2.version, 1);

    let app_2_ep_1 = create_test_endpoint(&client, &app_2, EP_URI_APP_2_EP_1)
        .await
        .unwrap();
    assert_eq!(app_2_ep_1.url, EP_URI_APP_2_EP_1);
    assert_eq!(app_2_ep_1.version, 1);

    let app_2_ep_2 = create_test_endpoint(&client, &app_2, EP_URI_APP_2_EP_2)
        .await
        .unwrap();
    assert_eq!(app_2_ep_2.url, EP_URI_APP_2_EP_2);
    assert_eq!(app_2_ep_2.version, 1);

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
            &format!("api/v1/app/{}/endpoint/{}/", app_1, app_1_ep_1_id),
            endpoint_in(EP_URI_APP_1_EP_1_VER_2),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_2);

    // CONFIRM UPDATE
    assert_eq!(
        get_endpoint(&client, &app_1, &app_1_ep_1_id).await.unwrap(),
        app_1_ep_1
    );

    // LIST
    let list_app_1: ListResponse<EndpointOut> = client
        .get(&format!("api/v1/app/{}/endpoint/", &app_1), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list_app_1.data.len(), 2);
    assert!(list_app_1.data.contains(&app_1_ep_1));
    assert!(list_app_1.data.contains(&app_1_ep_2));

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
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server();

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
    let (client, _jh) = start_svix_server();

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
            &format!("api/v1/app/{}/endpoint/", app_id),
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
    assert_eq!(ep_1.uid, ep_1_updated.uid);

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
    let (client, _jh) = start_svix_server();

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
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::INTERNAL_SERVER_ERROR);

    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let _: serde_json::Value = client
        .post(
            &format!("api/v1/app/{}/endpoint/{}/recover/", app_id, endp_id),
            RecoverIn {
                since: Utc::now() - chrono::Duration::weeks(3),
            },
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}

async fn create_failed_message_attempts() -> (
    TestClient,
    ApplicationId,
    EndpointId,
    MessageOut,
    MessageOut,
    usize,
    [DateTime<Utc>; 3],
) {
    let mut cfg = get_default_test_config();
    cfg.retry_schedule = (0..2)
        .into_iter()
        .map(|_| Duration::from_millis(1))
        .collect();

    let (client, _jh) = start_svix_server_with_cfg(&cfg);

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let receiver = TestReceiver::start(StatusCode::INTERNAL_SERVER_ERROR);

    let endp_id = create_test_endpoint(&client, &app_id, &receiver.endpoint)
        .await
        .unwrap()
        .id;

    let before_msg_1 = Utc::now();

    let msg_1 = create_test_message(&client, &app_id, serde_json::json!({"test": "data1"}))
        .await
        .unwrap();

    wait_for_msg_retries(&cfg.retry_schedule).await;

    let before_msg_2 = Utc::now();

    let msg_2 = create_test_message(&client, &app_id, serde_json::json!({"test": "data2"}))
        .await
        .unwrap();

    wait_for_msg_retries(&cfg.retry_schedule).await;

    let after_msg_2 = Utc::now();

    receiver.jh.abort();

    (
        client,
        app_id,
        endp_id,
        msg_1,
        msg_2,
        cfg.retry_schedule.len() + 1,
        [before_msg_1, before_msg_2, after_msg_2],
    )
}

#[tokio::test]
async fn test_recovery_expected_retry_counts() {
    sleep(Duration::from_millis(50)).await;

    for (i, (msg_1_retry_cnt, msg2_retry_cnt)) in [
        // expected number of additional retry attempts for (msg1, msg2) if recover `since` is before msg 1:
        (1, 1),
        // expected values if recover `since` is before msg 2:
        (0, 1),
        // expected values if recover `since` is after msg 2:
        (0, 0),
    ]
    .iter()
    .enumerate()
    {
        let (client, app_id, endp_id, msg_1, msg_2, base_attempt_cnt, times) =
            create_failed_message_attempts().await;

        recover_webhooks(
            &client,
            times[i],
            &format!("api/v1/app/{}/endpoint/{}/recover/", app_id, endp_id),
        )
        .await;

        sleep(Duration::from_millis(50)).await;

        get_msg_attempt_list_and_assert_count(
            &client,
            &app_id,
            &msg_1.id,
            base_attempt_cnt + msg_1_retry_cnt,
        )
        .await
        .unwrap();
        get_msg_attempt_list_and_assert_count(
            &client,
            &app_id,
            &msg_2.id,
            base_attempt_cnt + msg2_retry_cnt,
        )
        .await
        .unwrap();
    }
}
