// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(rename = "accessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
}

impl AzureBlobStorageConfigPatch {
    pub fn new() -> Self {
        Self {
            container: None,
            account: None,
            access_key: None,
        }
    }
}

impl Default for AzureBlobStorageConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
