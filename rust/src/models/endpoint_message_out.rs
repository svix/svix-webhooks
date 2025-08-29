// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, message_status_text::MessageStatusText};

/// A model containing information on a given message plus additional fields on
/// the last attempt for that message.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointMessageOut {
    /// List of free-form identifiers that endpoints can filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    /// Optional unique identifier for the message
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    /// The Message's ID.
    pub id: String,

    #[serde(rename = "nextAttempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<String>,

    pub payload: serde_json::Value,

    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    pub timestamp: String,
}

impl EndpointMessageOut {
    pub fn new(
        event_type: String,
        id: String,
        payload: serde_json::Value,
        status: MessageStatus,
        status_text: MessageStatusText,
        timestamp: String,
    ) -> Self {
        Self {
            channels: None,
            event_id: None,
            event_type,
            id,
            next_attempt: None,
            payload,
            status,
            status_text,
            tags: None,
            timestamp,
        }
    }
}
