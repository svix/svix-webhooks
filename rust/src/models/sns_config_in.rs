// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a SNS sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnsConfigIn {
    #[serde(rename = "topicArn")]
    pub topic_arn: String,

    /// The region of the SNS instance.
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

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

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl SnsConfigIn {
    pub fn new(topic_arn: String) -> Self {
        Self {
            topic_arn,
            region: None,
            access_key_id: None,
            secret_access_key: None,
            endpoint_url: None,
        }
    }
}
