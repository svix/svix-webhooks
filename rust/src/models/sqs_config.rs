// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for an SQS sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SqsConfig {
    #[serde(rename = "queueUrl")]
    pub queue_url: String,

    pub region: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl SqsConfig {
    pub fn new(
        queue_url: String,
        region: String,
        access_key_id: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            queue_url,
            region,
            access_key_id,
            secret_access_key,
            endpoint_url: None,
        }
    }
}
