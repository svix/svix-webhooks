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
