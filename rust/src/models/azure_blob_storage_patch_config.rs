// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStoragePatchConfig {
    #[serde(rename = "accessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
}

impl AzureBlobStoragePatchConfig {
    pub fn new() -> Self {
        Self {
            access_key: None,
            account: None,
            container: None,
        }
    }
}
