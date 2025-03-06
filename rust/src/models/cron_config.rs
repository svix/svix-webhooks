// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CronConfig {
    /// Override the default content-type.
    ///
    /// Recommended if the payload is not JSON.
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    pub payload: String,

    pub schedule: String,
}

impl CronConfig {
    pub fn new(payload: String, schedule: String) -> Self {
        Self {
            content_type: None,
            payload,
            schedule,
        }
    }
}
