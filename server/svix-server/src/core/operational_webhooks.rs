// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Module defining an interface for sending webhook events about the service.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use svix::api::{MessageIn, Svix, SvixOptions};

use super::{
    security::{generate_management_token, Keys},
    types::{
        ApplicationId, ApplicationUid, EndpointId, EndpointUid, MessageAttemptId, MessageId,
        MessageUid, OrganizationId,
    },
};
use crate::{
    db::models::{endpoint, messageattempt},
    error::{HttpError, Result},
};

/// Sent when an endpoint has been automatically disabled after continuous failures.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDisabledEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
    pub fail_since: DateTime<Utc>,
}

/// Sent when an endpoint is created, updated, or deleted
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub endpoint_id: EndpointId,
    pub endpoint_uid: Option<EndpointUid>,
}

impl EndpointEvent {
    pub fn new(app_uid: Option<&ApplicationUid>, endp: &endpoint::Model) -> Self {
        Self {
            app_id: endp.app_id.clone(),
            app_uid: app_uid.cloned(),
            endpoint_id: endp.id.clone(),
            endpoint_uid: endp.uid.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttempetLast {
    pub id: MessageAttemptId,
    pub response_status_code: i16,
    pub timestamp: DateTime<Utc>,
}

impl From<messageattempt::Model> for MessageAttempetLast {
    fn from(attempt: messageattempt::Model) -> Self {
        Self {
            id: attempt.id,
            response_status_code: attempt.response_status_code,
            timestamp: attempt.created_at.into(),
        }
    }
}

/// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a
/// "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing"
/// event.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttemptEvent {
    pub app_id: ApplicationId,
    pub app_uid: Option<ApplicationUid>,
    pub msg_id: MessageId,
    pub msg_event_id: Option<MessageUid>,
    pub endpoint_id: EndpointId,
    pub last_attempt: MessageAttempetLast,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum OperationalWebhook {
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

pub type OperationalWebhookSender = Arc<OperationalWebhookSenderInner>;

pub struct OperationalWebhookSenderInner {
    keys: Keys,
    url: Option<String>,
}

impl OperationalWebhookSenderInner {
    pub fn new(keys: Keys, url: Option<String>) -> Arc<Self> {
        Arc::new(Self { keys, url })
    }

    pub async fn send_operational_webhook(
        &self,
        recipient_org_id: &OrganizationId,
        payload: OperationalWebhook,
    ) -> Result<()> {
        let url = match self.url.as_ref() {
            Some(url) => url,
            None => return Ok(()),
        };

        let op_webhook_token =
            generate_management_token(&self.keys).expect("Error generating Svix Management token");
        let svix_api = Svix::new(
            op_webhook_token,
            Some(SvixOptions {
                server_url: Some(url.to_string()),
                debug: false,
            }),
        );

        let payload = serde_json::to_value(payload)
            .map_err(|_| HttpError::internal_server_error(None, None))?;

        // Get the event type from the type field
        let event_type: String = payload
            .get("type")
            .ok_or_else(|| HttpError::internal_server_error(None, None))?
            .as_str()
            .ok_or_else(|| HttpError::internal_server_error(None, None))?
            .to_string();

        let recipient_org_id = recipient_org_id.to_string();

        tokio::spawn(async move {
            // This sends a webhook under the Svix management organization. This organization contains
            // applications which are each a regular organization. The recipient's OrganizationId is the
            // app UID to use.
            let resp = svix_api
                .message()
                .create(
                    recipient_org_id.clone(),
                    MessageIn {
                        event_type,
                        payload,
                        ..MessageIn::default()
                    },
                    None,
                )
                .await;

            match resp {
                Ok(_) => {}
                // Ignore 404s because not every org will have an associated application
                Err(svix::error::Error::Http(svix::error::HttpErrorContent {
                    status: StatusCode::NOT_FOUND,
                    ..
                })) => {
                    tracing::warn!(
                        "Operational webhooks are enabled but no listener set for {}",
                        recipient_org_id,
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Failed sending operational webhook for {} {}",
                        recipient_org_id,
                        e.to_string()
                    );
                }
            }
        });

        Ok(())
    }
}
