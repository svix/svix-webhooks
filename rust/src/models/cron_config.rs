// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CronConfig {
    pub schedule: String,

    pub payload: String,

    /// Override the default content-type.
    ///
    /// Recommended if the payload is not JSON.
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl CronConfig {
    pub fn new(schedule: String, payload: String) -> Self {
        Self {
            schedule,
            payload,
            content_type: None,
        }
    }
}
