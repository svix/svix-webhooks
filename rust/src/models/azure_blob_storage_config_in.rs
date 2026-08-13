// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfigIn {
    pub container: String,

    pub account: String,

    /// Access key.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "accessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
}

impl AzureBlobStorageConfigIn {
    pub fn new(container: String, account: String) -> Self {
        Self {
            container,
            account,
            access_key: None,
        }
    }
}
