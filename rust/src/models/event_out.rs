// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventOut {
    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    pub payload: String,

    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl EventOut {
    pub fn new(
        event_type: String,
        payload: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            event_type,
            payload,
            timestamp,
        }
    }
}
