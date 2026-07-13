// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a SNS sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnsConfig {
    #[serde(rename = "topicArn")]
    pub topic_arn: String,

    pub region: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl SnsConfig {
    pub fn new(
        topic_arn: String,
        region: String,
        access_key_id: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            topic_arn,
            region,
            access_key_id,
            secret_access_key,
            endpoint_url: None,
        }
    }
}
