// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for an SQS sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SqsConfig {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    #[serde(rename = "queueUrl")]
    pub queue_url: String,

    pub region: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
}

impl SqsConfig {
    pub fn new(
        access_key_id: String,
        queue_url: String,
        region: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            access_key_id,
            endpoint_url: None,
            queue_url,
            region,
            secret_access_key,
        }
    }
}
