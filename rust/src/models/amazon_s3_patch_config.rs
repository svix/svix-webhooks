// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AmazonS3PatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl AmazonS3PatchConfig {
    pub fn new() -> Self {
        Self {
            bucket: None,
            access_key_id: None,
            secret_access_key: None,
            region: None,
            endpoint_url: None,
        }
    }
}

impl Default for AmazonS3PatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
