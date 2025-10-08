// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudStoragePatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}

impl GoogleCloudStoragePatchConfig {
    pub fn new() -> Self {
        Self {
            bucket: None,
            credentials: None,
        }
    }
}
