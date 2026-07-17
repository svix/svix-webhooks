// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, message_status_text::MessageStatusText};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEndpointOut {
    /// The Endpoint's ID.
    pub id: String,

    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    #[serde(rename = "nextAttempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<String>,

    pub url: String,

    pub description: String,

    /// Maximum messages per second to send to this endpoint.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    /// List of message channels this endpoint listens to (omit for all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl MessageEndpointOut {
    pub fn new(
        id: String,
        status: MessageStatus,
        status_text: MessageStatusText,
        url: String,
        description: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            status,
            status_text,
            next_attempt: None,
            url,
            description,
            throttle_rate: None,
            uid: None,
            disabled: None,
            event_types: None,
            channels: None,
            created_at,
            updated_at,
        }
    }
}
