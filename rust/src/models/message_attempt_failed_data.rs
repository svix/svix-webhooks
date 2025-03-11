// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptFailedData {
    /// The MessageAttempt's ID.
    pub id: String,

    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i16,

    pub timestamp: String,
}

impl MessageAttemptFailedData {
    pub fn new(id: String, response_status_code: i16, timestamp: String) -> Self {
        Self {
            id,
            response_status_code,
            timestamp,
        }
    }
}
