// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use anyhow::Result;
use reqwest::StatusCode;

use serde::Serialize;
use svix_server::{
    core::types::{ApplicationId, EventTypeName},
    v1::endpoints::{
        application::{ApplicationIn, ApplicationOut},
        endpoint::{EndpointIn, EndpointOut},
        message::{MessageIn, MessageOut},
    },
};

use super::{IgnoredResponse, TestClient};

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
        .delete(&format!("api/v1/app/{}/", id), StatusCode::NO_CONTENT)
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
            &format!("api/v1/app/{}/endpoint/", app_id),
            ep,
            StatusCode::CREATED,
        )
        .await
}

// Message

pub fn message_in<T: Serialize>(event_type: &str, payload: T) -> Result<MessageIn> {
    Ok(MessageIn {
        event_type: EventTypeName(event_type.to_owned()),
        payload: serde_json::to_value(payload)?,

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
