// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

/// The MessageOut equivalent of polling endpoint
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointMessageOut {
    /// List of free-form identifiers that endpoints can filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(rename = "deliverAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_at: Option<String>,

    /// Optional unique identifier for the message
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// The Message's ID.
    pub id: String,

    pub payload: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    pub timestamp: String,
}

impl PollingEndpointMessageOut {
    pub fn new(
        event_type: String,
        id: String,
        payload: serde_json::Value,
        timestamp: String,
    ) -> Self {
        Self {
            channels: None,
            deliver_at: None,
            event_id: None,
            event_type,
            headers: None,
            id,
            payload,
            tags: None,
            timestamp,
        }
    }
}
