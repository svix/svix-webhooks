// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfig {
    pub container: String,

    pub account: String,

    #[serde(rename = "accessKey")]
    pub access_key: String,
}

impl AzureBlobStorageConfig {
    pub fn new(container: String, account: String, access_key: String) -> Self {
        Self {
            container,
            account,
            access_key,
        }
    }
}
