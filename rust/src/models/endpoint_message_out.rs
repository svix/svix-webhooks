// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, message_status_text::MessageStatusText};

/// A model containing information on a given message plus additional fields on
/// the last attempt for that message.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointMessageOut {
    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    #[serde(rename = "nextAttempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<String>,

    /// Optional unique identifier for the message
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    pub payload: serde_json::Value,

    /// List of free-form identifiers that endpoints can filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    /// The Message's ID.
    pub id: String,

    pub timestamp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "deliverAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_at: Option<String>,
}

impl EndpointMessageOut {
    pub fn new(
        status: MessageStatus,
        status_text: MessageStatusText,
        event_type: String,
        payload: serde_json::Value,
        id: String,
        timestamp: String,
    ) -> Self {
        Self {
            status,
            status_text,
            next_attempt: None,
            event_id: None,
            event_type,
            payload,
            channels: None,
            id,
            timestamp,
            tags: None,
            deliver_at: None,
        }
    }
}
