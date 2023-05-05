// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{net::TcpListener, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use http::StatusCode;

use reqwest::Url;
use serde::Deserialize;
use svix::api::EventTypeOut;
use svix_ksuid::KsuidLike;
use svix_server::{
    cfg::ConfigurationInner,
    core::{
        security::{generate_org_token, management_org_id},
        types::{
            metadata::Metadata, ApplicationId, ApplicationUid, BaseId, EndpointId, EndpointUid,
            MessageAttemptId, MessageId, MessageUid, OrganizationId,
        },
    },
    v1::endpoints::{
        application::{ApplicationIn, ApplicationOut},
        endpoint::{EndpointIn, EndpointOut, EndpointSecretRotateIn},
    },
};

mod utils;
use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message},
    get_default_test_config, IgnoredResponse, TestClient, TestReceiver,
};

use crate::utils::common_calls::default_test_endpoint;

/// Sent when an endpoint has been automatically disabled after continuous failures.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDisabledEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
    pub fail_since: DateTime<Utc>,
}

/// Sent when an endpoint is created, updated, or deleted
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttempetLast {
    pub id: MessageAttemptId,
    pub response_status_code: i16,
    pub timestamp: DateTime<Utc>,
}

/// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a
/// "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing"
/// event.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttemptEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub msg_id: MessageId,
    pub msg_event_id: Option<MessageUid>,
    pub endpoint_id: EndpointId,
    pub last_attempt: MessageAttempetLast,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum OperationalWebhookTest {
    #[serde(rename = "endpoint.disabled")]
    EndpointDisabled(EndpointDisabledEvent),
    #[serde(rename = "endpoint.created")]
    EndpointCreated(EndpointEvent),
    #[serde(rename = "endpoint.updated")]
    EndpointUpdated(EndpointEvent),
    #[serde(rename = "endpoint.deleted")]
    EndpointDeleted(EndpointEvent),
    #[serde(rename = "message.attempt.exhausted")]
    MessageAttemptExhausted(MessageAttemptEvent),
    #[serde(rename = "message.attempt.failing")]
    MessageAttemptFailing(MessageAttemptEvent),
}

/// Operational webhooks are dispatched by a special organization, so this function returns two
/// [`TestClient`]s, one with the Svix Management org token, and one with a random org token.
///
/// Additionally it returns the [`OrganizationId`] of the random  organization such that it may be
/// included as an application of the of the Operational webhooks organization.
fn start_svix_server_with_operational_webhooks(
    mut cfg: ConfigurationInner,
) -> (
    TestClient,
    TestClient,
    OrganizationId,
    tokio::task::JoinHandle<()>,
) {
    let op_webhook_jwt = generate_org_token(&cfg.jwt_secret, management_org_id()).unwrap();

    let org_id = OrganizationId::new(None, None);
    let regular_jwt = generate_org_token(&cfg.jwt_secret, org_id.clone()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());

    cfg.operational_webhook_address = Some(base_url.clone());
    let cfg = Arc::new(cfg);

    let jh = tokio::spawn(svix_server::run_with_prefix(
        Some(svix_ksuid::Ksuid::new(None, None).to_string()),
        cfg,
        Some(listener),
    ));

    (
        TestClient::new(base_url.clone(), &regular_jwt),
        TestClient::new(base_url, &op_webhook_jwt),
        org_id,
        jh,
    )
}

#[tokio::test]
async fn test_endpoint_create_update_and_delete() {
    let cfg = get_default_test_config();

    let (client_regular, client_op, org_id, _jh) = start_svix_server_with_operational_webhooks(cfg);

    // Setup operational webhook Application and Endpoint
    let op_webhook_app: ApplicationOut = client_op
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "TestOperationalWebhookApplication".to_owned(),
                rate_limit: None,
                uid: Some(ApplicationUid(org_id.to_string())),
                metadata: Metadata::default(),
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let _op_webhook_endp: EndpointOut = client_op
        .post(
            &format!("api/v1/app/{}/endpoint/", op_webhook_app.id),
            EndpointIn {
                description: "TestOperationalWebhookEndpoint".to_owned(),
                url: Url::parse(&receiver.endpoint).unwrap(),
                ..default_test_endpoint()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Setup regular Application and Endpoint
    let regular_app = create_test_app(&client_regular, "TestOperationalWebhookApplicationRegular")
        .await
        .unwrap();
    let regular_endp = create_test_endpoint(&client_regular, &regular_app.id, "http://junk.url")
        .await
        .unwrap();

    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "endpoint.created"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    match op_webhook_out {
        OperationalWebhookTest::EndpointCreated(EndpointEvent {
            app_id,
            app_uid,
            endpoint_id,
            endpoint_uid,
        }) => {
            assert_eq!(app_id, regular_app.id);
            assert_eq!(app_uid, regular_app.uid);
            assert_eq!(endpoint_id, regular_endp.id);
            assert_eq!(endpoint_uid, regular_endp.ep.uid);
        }
        _ => panic!("Got wrong type"),
    };

    // Update endpoint
    let regular_endp: EndpointOut = client_regular
        .put(
            &format!(
                "api/v1/app/{}/endpoint/{}/",
                regular_app.id, regular_endp.id
            ),
            EndpointIn {
                description: "Updated description".to_owned(),
                url: Url::parse(&receiver.endpoint).unwrap(),
                version: 2,
                ..default_test_endpoint()
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "endpoint.updated"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    match op_webhook_out {
        OperationalWebhookTest::EndpointUpdated(EndpointEvent {
            app_id,
            app_uid,
            endpoint_id,
            endpoint_uid,
        }) => {
            assert_eq!(app_id, regular_app.id);
            assert_eq!(app_uid, regular_app.uid);
            assert_eq!(endpoint_id, regular_endp.id);
            assert_eq!(endpoint_uid, regular_endp.ep.uid);
        }
        _ => panic!("Got wrong type"),
    };

    // Rotate secrets
    let _: IgnoredResponse = client_regular
        .post(
            &format!(
                "api/v1/app/{}/endpoint/{}/secret/rotate/",
                regular_app.id, regular_endp.id
            ),
            EndpointSecretRotateIn::default(),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "endpoint.updated"
    );

    // And finally delete the endpoint
    let _: IgnoredResponse = client_regular
        .delete(
            &format!(
                "api/v1/app/{}/endpoint/{}/",
                regular_app.id, regular_endp.id
            ),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "endpoint.deleted"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    match op_webhook_out {
        OperationalWebhookTest::EndpointDeleted(EndpointEvent {
            app_id,
            app_uid,
            endpoint_id,
            endpoint_uid,
        }) => {
            assert_eq!(app_id, regular_app.id);
            assert_eq!(app_uid, regular_app.uid);
            assert_eq!(endpoint_id, regular_endp.id);
            assert_eq!(endpoint_uid, regular_endp.ep.uid);
        }
        _ => panic!("Got wrong type"),
    };
}

#[tokio::test]
async fn test_message_attempt_operational_webhooks() {
    let mut cfg = get_default_test_config();

    cfg.retry_schedule = (0..5).map(|_| Duration::from_millis(1)).collect();

    let (client_regular, client_op, org_id, _jh) = start_svix_server_with_operational_webhooks(cfg);

    // Setup operational webhook Application and Endpoint
    let op_webhook_app: ApplicationOut = client_op
        .post(
            "api/v1/app/",
            ApplicationIn {
                name: "TestOperationalWebhookApplication".to_owned(),
                rate_limit: None,
                uid: Some(ApplicationUid(org_id.to_string())),
                metadata: Metadata::default(),
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let _op_webhook_endp: EndpointOut = client_op
        .post(
            &format!("api/v1/app/{}/endpoint/", op_webhook_app.id),
            EndpointIn {
                description: "TestOperationalWebhookEndpoint".to_owned(),
                url: Url::parse(&receiver.endpoint).unwrap(),
                ..default_test_endpoint()
            },
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    // Setup regular Application and Endpoint
    let regular_app = create_test_app(&client_regular, "TestOperationalWebhookApplicationRegular")
        .await
        .unwrap();
    let regular_endp = create_test_endpoint(&client_regular, &regular_app.id, "http://junk.url")
        .await
        .unwrap();

    // Receive the EndpointCreatedEvent so it's not in the data_recv queue
    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "endpoint.created"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    match op_webhook_out {
        OperationalWebhookTest::EndpointCreated(EndpointEvent {
            app_id,
            app_uid,
            endpoint_id,
            endpoint_uid,
        }) => {
            assert_eq!(app_id, regular_app.id);
            assert_eq!(app_uid, regular_app.uid);
            assert_eq!(endpoint_id, regular_endp.id);
            assert_eq!(endpoint_uid, regular_endp.ep.uid);
        }
        _ => panic!("Got wrong type"),
    };

    // Create a message
    let regular_msg = create_test_message(
        &client_regular,
        &regular_app.id,
        serde_json::json!({"test": "data"}),
    )
    .await
    .unwrap();

    // Wait a little to ensure they've been processed
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Assert first the MessageAttemptFailingEvent was received then the MessageAttemptExhaustedEvent
    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "message.attempt.failing"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    if let OperationalWebhookTest::MessageAttemptFailing(MessageAttemptEvent {
        app_id,
        app_uid,
        msg_id,
        endpoint_id,
        // Rest aren't practical to test
        ..
    }) = op_webhook_out
    {
        assert_eq!(app_id, regular_app.id);
        assert_eq!(app_uid, regular_app.uid);
        assert_eq!(msg_id, regular_msg.id);
        assert_eq!(endpoint_id, regular_endp.id);
    } else {
        panic!("Invalid type for op_webhook_out")
    }

    let op_webhook_out = receiver.data_recv.recv().await.unwrap();
    assert_eq!(
        op_webhook_out.get("type").unwrap().as_str().unwrap(),
        "message.attempt.exhausted"
    );
    let op_webhook_out: OperationalWebhookTest = serde_json::from_value(op_webhook_out).unwrap();

    if let OperationalWebhookTest::MessageAttemptExhausted(MessageAttemptEvent {
        app_id,
        app_uid,
        msg_id,
        endpoint_id,
        // Rest aren't practical to test
        ..
    }) = op_webhook_out
    {
        assert_eq!(app_id, regular_app.id);
        assert_eq!(app_uid, regular_app.uid);
        assert_eq!(msg_id, regular_msg.id);
        assert_eq!(endpoint_id, regular_endp.id);
    } else {
        panic!("Invalid type for op_webhook_out")
    }
}

#[tokio::test]
async fn test_operational_webhooks_event_types_exist() {
    let cfg = get_default_test_config();
    let (_, client_op, _, _jh) = start_svix_server_with_operational_webhooks(cfg);

    for et in &[
        "message.attempt.failing",
        "message.attempt.exhausted",
        "endpoint.created",
        "endpoint.deleted",
        "endpoint.disabled",
        "endpoint.created",
        "endpoint.updated",
    ] {
        let _: EventTypeOut = client_op
            .get(&format!("api/v1/event-type/{et}/"), StatusCode::OK)
            .await
            .unwrap();
    }
}
