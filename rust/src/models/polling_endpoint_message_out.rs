// this file is @generated
use serde::{Deserialize, Serialize};

/// The MessageOut equivalent of polling endpoint
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointMessageOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,

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
    pub channels: Option<std::collections::BTreeSet<String>>,

    /// The Message's ID.
    pub id: String,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeSet<String>>,

    #[serde(rename = "deliverAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl PollingEndpointMessageOut {
    pub fn new(
        event_type: String,
        payload: serde_json::Value,
        id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            headers: None,
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
