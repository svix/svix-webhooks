// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStoragePatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(rename = "accessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
}

impl AzureBlobStoragePatchConfig {
    pub fn new() -> Self {
        Self {
            container: None,
            account: None,
            access_key: None,
        }
    }
}

impl Default for AzureBlobStoragePatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
