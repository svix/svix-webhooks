// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3Config {
    pub bucket: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    pub region: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl S3Config {
    pub fn new(
        bucket: String,
        access_key_id: String,
        secret_access_key: String,
        region: String,
    ) -> Self {
        Self {
            bucket,
            access_key_id,
            secret_access_key,
            region,
            endpoint_url: None,
        }
    }
}
