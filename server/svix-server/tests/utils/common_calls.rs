// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;

use serde::{de::DeserializeOwned, Serialize};
use svix_server::{
    core::types::{metadata::Metadata, ApplicationId, EventTypeName, MessageId},
    v1::{
        endpoints::{
            application::{ApplicationIn, ApplicationOut},
            attempt::MessageAttemptOut,
            endpoint::{EndpointIn, EndpointOut, RecoverIn},
            event_type::EventTypeIn,
            message::{MessageIn, MessageOut},
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

pub fn endpoint_in(url: &str) -> EndpointIn {
    EndpointIn {
        url: url.to_owned(),
        version: 1,
        ..Default::default()
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
        payload: serde_json::to_value(payload)?,
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

pub fn event_type_in(
    name: &str,
    schema: impl Into<Option<serde_json::Value>>,
) -> Result<EventTypeIn> {
    Ok(EventTypeIn {
        name: EventTypeName(name.to_owned()),
        description: "test-event-description".to_owned(),
        deleted: false,
        schemas: schema.into().map(|s| serde_json::from_value(s).unwrap()),
    })
}

// Common tests
pub async fn common_test_list<
    ModelOut: DeserializeOwned + Clone + PartialEq + std::fmt::Debug,
    ModelIn: Serialize,
>(
    client: &TestClient,
    path: &str,
    create_model: fn(usize) -> ModelIn,
    sort_asc: bool,
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

    let list = run_with_retries(|| async {
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
            assert_eq!(items.get(i), list.data.get(i));
        }
    } else {
        for i in 0..10 {
            assert_eq!(items.get(10 - i), list.data.get(i));
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

    let _list = client
        .get::<IgnoredResponse>(
            &format!("{path}?limit=6&iterator=BAD-$$$ITERATOR"),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

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
    let _: serde_json::Value = client
        .post(url, RecoverIn { since }, StatusCode::ACCEPTED)
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
