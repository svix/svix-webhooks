// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventIn {
    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    pub payload: String,
}

impl EventIn {
    pub fn new(event_type: String, payload: String) -> Self {
        Self {
            event_type,
            payload,
        }
    }
}
