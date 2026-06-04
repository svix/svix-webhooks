// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a SNS sink.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SnsConfig {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    pub region: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

impl SnsConfig {
    pub fn new(
        access_key_id: String,
        region: String,
        secret_access_key: String,
        topic_arn: String,
    ) -> Self {
        Self {
            access_key_id,
            endpoint_url: None,
            region,
            secret_access_key,
            topic_arn,
        }
    }
}
