// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, time::Duration};

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{StatusCode, Url};

use serde::{de::DeserializeOwned, Serialize};
use svix::api::DashboardAccessOut;
use svix_server::{
    core::types::{
        metadata::Metadata, ApplicationId, EventChannel, EventChannelSet, EventTypeName,
        FeatureFlagSet, MessageId,
    },
    v1::{
        endpoints::{
            application::{ApplicationIn, ApplicationOut},
            attempt::MessageAttemptOut,
            auth::AppPortalAccessIn,
            endpoint::{EndpointIn, EndpointOut, RecoverIn},
            event_type::EventTypeIn,
            message::{MessageIn, MessageOut, RawPayload},
        },
        utils::ListResponse,
    },
};

use super::{run_with_retries, IgnoredResponse, TestClient};

// App

pub fn application_in(name: &str) -> ApplicationIn {
    ApplicationIn {
        name: name.to_owned(),
        ..Default::default()
    }
}

pub async fn create_test_app(client: &TestClient, name: &str) -> Result<ApplicationOut> {
    client
        .post("api/v1/app/", application_in(name), StatusCode::CREATED)
        .await
}

pub async fn delete_test_app(client: &TestClient, id: ApplicationId) -> Result<IgnoredResponse> {
    client
        .delete(&format!("api/v1/app/{id}/"), StatusCode::NO_CONTENT)
        .await
}

// Endpoint

pub fn default_test_endpoint() -> EndpointIn {
    EndpointIn {
        description: Default::default(),
        rate_limit: Default::default(),
        uid: Default::default(),
        url: Url::parse("http://example.com").unwrap(),
        version: 1,
        disabled: Default::default(),
        event_types_ids: Default::default(),
        channels: Default::default(),
        key: Default::default(),
        metadata: Default::default(),
    }
}

pub fn endpoint_in(url: &str) -> EndpointIn {
    EndpointIn {
        url: Url::parse(url).unwrap(),
        ..default_test_endpoint()
    }
}

pub async fn create_test_endpoint(
    client: &TestClient,
    app_id: &ApplicationId,
    url: &str,
) -> Result<EndpointOut> {
    post_endpoint(client, app_id, endpoint_in(url)).await
}

pub async fn post_endpoint(
    client: &TestClient,
    app_id: &str,
    ep: EndpointIn,
) -> Result<EndpointOut> {
    client
        .post(
            &format!("api/v1/app/{app_id}/endpoint/"),
            ep,
            StatusCode::CREATED,
        )
        .await
}

pub async fn put_endpoint(
    client: &TestClient,
    app_id: &str,
    ep_id: &str,
    ep: EndpointIn,
) -> Result<EndpointOut> {
    client
        .put(
            &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
            ep,
            StatusCode::OK,
        )
        .await
}

// Message

pub fn message_in<T: Serialize>(event_type: &str, payload: T) -> Result<MessageIn> {
    Ok(MessageIn {
        event_type: EventTypeName(event_type.to_owned()),
        payload: RawPayload::from_string(serde_json::to_string(&payload)?)?,
        payload_retention_period: 5,
        channels: None,
        uid: None,
    })
}

pub async fn create_test_message(
    client: &TestClient,
    app_id: &ApplicationId,
    payload: serde_json::Value,
) -> Result<MessageOut> {
    client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in("event.type", payload)?,
            StatusCode::ACCEPTED,
        )
        .await
}

pub async fn create_test_msg_with(
    client: &TestClient,
    app_id: &ApplicationId,
    payload: serde_json::Value,
    event_type: &str,
    channel: impl IntoIterator<Item = &str>,
) -> MessageOut {
    let channels: HashSet<EventChannel> = channel
        .into_iter()
        .map(|x| EventChannel(x.to_string()))
        .collect();

    let channels = if channels.is_empty() {
        None
    } else {
        Some(EventChannelSet(channels))
    };

    client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            MessageIn {
                event_type: EventTypeName(event_type.to_owned()),
                payload: RawPayload::from_string(serde_json::to_string(&payload).unwrap()).unwrap(),
                payload_retention_period: 5,
                channels,
                uid: None,
            },
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap()
}

pub fn event_type_in(
    name: &str,
    schema: impl Into<Option<serde_json::Value>>,
) -> Result<EventTypeIn> {
    Ok(EventTypeIn {
        name: EventTypeName(name.to_owned()),
        description: "test-event-description".to_owned(),
        deleted: false,
        schemas: schema.into().map(|s| serde_json::from_value(s).unwrap()),
        feature_flag: None,
    })
}

// Common tests
pub async fn common_test_list<
    ModelOut: DeserializeOwned + PartialEq + std::fmt::Debug,
    ModelIn: Serialize,
>(
    client: &TestClient,
    path: &str,
    create_model: fn(usize) -> ModelIn,
    sort_asc: bool,
    supports_reverse: bool,
) -> Result<()> {
    let mut items = Vec::new();
    for i in 0..10 {
        let item: ModelOut = client
            .post(path, create_model(i), StatusCode::CREATED)
            .await
            .unwrap();
        // Sleep for 5ms because KsuidMs has 4ms accuracy so things got out of order
        tokio::time::sleep(Duration::from_millis(5)).await;
        items.push(item);
    }

    let original_list = run_with_retries(|| async {
        let list = client
            .get::<ListResponse<ModelOut>>(&format!("{path}?with_content=true"), StatusCode::OK)
            .await
            .unwrap();

        assert_eq!(list.data.len(), 10);

        Ok(list)
    })
    .await
    .unwrap();

    if sort_asc {
        for i in 0..10 {
            assert_eq!(items.get(i), original_list.data.get(i));
        }
    } else {
        for i in 0..10 {
            assert_eq!(items.get(9 - i), original_list.data.get(i));
        }
    }

    // Limit results
    let list = client
        .get::<ListResponse<ModelOut>>(&format!("{path}?limit=1"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 1);
    assert!(!list.done);

    let list = client
        .get::<ListResponse<ModelOut>>(&format!("{path}?limit=50"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 10);
    assert!(list.done);

    let list = client
        .get::<ListResponse<ModelOut>>(&format!("{path}?limit=10"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 10);
    assert!(list.done);

    let list = client
        .get::<ListResponse<ModelOut>>(&format!("{path}?limit=6"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 6);
    assert!(!list.done);

    let list = client
        .get::<ListResponse<ModelOut>>(
            &format!("{}?limit=6&iterator={}", path, list.iterator.unwrap()),
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(list.data.len(), 4);
    assert!(list.done);

    let prev = client
        .get::<ListResponse<ModelOut>>(
            &format!("{}?limit=3&iterator={}", path, list.prev_iterator.unwrap()),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(prev.data.len(), 3);
    assert_eq!(
        prev.data.first().unwrap(),
        original_list.data.get(3).unwrap()
    );

    let _list = client
        .get::<IgnoredResponse>(
            &format!("{path}?limit=6&iterator=BAD-$$$ITERATOR"),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

    if supports_reverse {
        let opposite_order = if sort_asc { "descending" } else { "ascending" };

        let opposite_1 = client
            .get::<ListResponse<ModelOut>>(
                &format!("{path}?limit=3&order={opposite_order}"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let opposite_2 = client
            .get::<ListResponse<ModelOut>>(
                &format!(
                    "{path}?limit=3&order={opposite_order}&iterator={}",
                    opposite_1.iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let opposite_prev = client
            .get::<ListResponse<ModelOut>>(
                &format!(
                    "{path}?limit=3&order={opposite_order}&iterator={}",
                    opposite_2.prev_iterator.unwrap()
                ),
                StatusCode::OK,
            )
            .await
            .unwrap();

        for i in 0..3 {
            assert_eq!(original_list.data.get(9 - i), opposite_1.data.get(i));
            assert_eq!(original_list.data.get(6 - i), opposite_2.data.get(i));
            assert_eq!(original_list.data.get(9 - i), opposite_prev.data.get(i));
        }
    }

    // Test limits -- ten models were created previously and the default hard/soft cap is 250 so
    // 10..251 is the sane range here.
    for i in 10..251 {
        let _: ModelOut = client
            .post(path, create_model(i), StatusCode::CREATED)
            .await
            .unwrap();
    }

    // If limits are hard, it will be a 422 UNPROCESSABLE_ENTITY response, otherwise it'll be capped
    // to 250
    if client
        .get::<IgnoredResponse>(
            &format!("{path}?limit=300"),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .is_ok()
        || client
            .get::<ListResponse<ModelOut>>(&format!("{path}?limit=300"), StatusCode::OK)
            .await
            .unwrap()
            .data
            .len()
            == 250
    {
        Ok(())
    } else {
        panic!("Error with soft/hard caps to pagination limits")
    }
}

pub async fn recover_webhooks(client: &TestClient, since: DateTime<Utc>, url: &str) {
    client
        .post_without_response(url, RecoverIn { since }, StatusCode::ACCEPTED)
        .await
        .unwrap();
}

pub async fn get_msg_attempt_list_and_assert_count(
    client: &TestClient,
    app_id: &ApplicationId,
    msg_id: &MessageId,
    expected_count: usize,
) -> Result<ListResponse<MessageAttemptOut>> {
    run_with_retries(|| async {
        let list: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/msg/{msg_id}/"),
                StatusCode::OK,
            )
            .await?;

        if list.data.len() != expected_count {
            anyhow::bail!(
                "Attempt count {} does not match expected length {}",
                list.data.len(),
                expected_count
            );
        }
        Ok(list)
    })
    .await
}

pub fn metadata(s: &str) -> Metadata {
    serde_json::from_str::<Metadata>(s).unwrap()
}

/// Accesses the app-portal-access endpoint and returns a new [`TestClient`] with an auth header set
/// to the returned token.
pub async fn app_portal_access(
    org_client: &TestClient,
    application_id: &ApplicationId,
    feature_flags: FeatureFlagSet,
) -> TestClient {
    let resp: DashboardAccessOut = org_client
        .post(
            &format!("api/v1/auth/app-portal-access/{application_id}/"),
            AppPortalAccessIn { feature_flags },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let mut app_client = org_client.clone();
    app_client.set_auth_header(resp.token);

    app_client
}
