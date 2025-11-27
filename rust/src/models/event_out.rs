// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventOut {
    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    pub payload: String,

    pub timestamp: String,
}

impl EventOut {
    pub fn new(
        event_type: String,
        payload: String,
        timestamp: String,
    ) -> Self {
        Self {
            event_type,
            payload,
            timestamp,
        }
    }
}
