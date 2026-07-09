// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SqsPatchConfig {
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    #[serde(rename = "queueUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

impl SqsPatchConfig {
    pub fn new() -> Self {
        Self {
            access_key_id: None,
            endpoint_url: None,
            queue_url: None,
            region: None,
            secret_access_key: None,
        }
    }
}

impl Default for SqsPatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
