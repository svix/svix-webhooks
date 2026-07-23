// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePrecheckIn {
    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<std::collections::BTreeSet<String>>,
}

impl MessagePrecheckIn {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            channels: None,
        }
    }
}
