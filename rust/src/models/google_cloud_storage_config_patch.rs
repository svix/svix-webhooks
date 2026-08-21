// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudStorageConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}

impl GoogleCloudStorageConfigPatch {
    pub fn new() -> Self {
        Self {
            bucket: None,
            credentials: None,
        }
    }
}

impl Default for GoogleCloudStorageConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
