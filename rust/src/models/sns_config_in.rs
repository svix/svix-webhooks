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
    /// Required (along with `secret_access_key`) if `role_arn` is None
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Required (along with `access_key_id`) if `role_arn` is None
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    /// Role ARN for delegated authentication
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    /// Shared secret passed as the STS ExternalId.
    ///
    /// Can only be set if `role_arn` is Some
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl SnsConfigIn {
    pub fn new(topic_arn: String) -> Self {
        Self {
            topic_arn,
            region: None,
            access_key_id: None,
            secret_access_key: None,
            endpoint_url: None,
            role_arn: None,
            external_id: None,
        }
    }
}
