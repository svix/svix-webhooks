// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageOut {
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

impl MessageOut {
    pub fn new(
        event_type: String,
        payload: serde_json::Value,
        id: String,
        timestamp: String,
    ) -> Self {
        Self {
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
