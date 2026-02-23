// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePrecheckIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,
}

impl MessagePrecheckIn {
    pub fn new(event_type: String) -> Self {
        Self {
            channels: None,
            event_type,
        }
    }
}
