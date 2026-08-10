// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfigIn {
    pub container: String,

    pub account: String,

    #[serde(rename = "accessKey")]
    pub access_key: String,
}

impl AzureBlobStorageConfigIn {
    pub fn new(container: String, account: String, access_key: String) -> Self {
        Self {
            container,
            account,
            access_key,
        }
    }
}
