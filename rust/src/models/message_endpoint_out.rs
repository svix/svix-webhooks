// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, message_status_text::MessageStatusText};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageEndpointOut {
    /// List of message channels this endpoint listens to (omit for all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// An example endpoint name.
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    /// The Endpoint's ID.
    pub id: String,

    #[serde(rename = "nextAttempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<String>,

    /// Deprecated, use `throttleRate` instead.
    #[deprecated]
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    /// Maximum messages per second to send to this endpoint. Outgoing messages
    /// will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub url: String,

    #[deprecated]
    pub version: i32,
}

impl MessageEndpointOut {
    pub fn new(
        created_at: String,
        description: String,
        id: String,
        status: MessageStatus,
        status_text: MessageStatusText,
        updated_at: String,
        url: String,
        version: i32,
    ) -> Self {
        #[allow(deprecated)]
        Self {
            channels: None,
            created_at,
            description,
            disabled: None,
            filter_types: None,
            id,
            next_attempt: None,
            rate_limit: None,
            status,
            status_text,
            throttle_rate: None,
            uid: None,
            updated_at,
            url,
            version,
        }
    }
}
