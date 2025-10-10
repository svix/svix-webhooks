// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfig {
    #[serde(rename = "accessKey")]
    pub access_key: String,

    pub account: String,

    pub container: String,
}

impl AzureBlobStorageConfig {
    pub fn new(access_key: String, account: String, container: String) -> Self {
        Self {
            access_key,
            account,
            container,
        }
    }
}
