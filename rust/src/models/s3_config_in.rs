// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3ConfigIn {
    pub bucket: String,

    /// Access key ID.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    /// The region of the EventBridge bus.
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl S3ConfigIn {
    pub fn new(bucket: String) -> Self {
        Self {
            bucket,
            access_key_id: None,
            secret_access_key: None,
            region: None,
            endpoint_url: None,
        }
    }
}
