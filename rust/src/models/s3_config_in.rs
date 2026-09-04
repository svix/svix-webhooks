// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3ConfigIn {
    pub bucket: String,

    /// Access key ID.
    ///
    /// Required (along with `secret_access_key`) if `role_arn` is null
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Required (along with `access_key_id`) if `role_arn` is null
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    /// The region of the S3 bucket
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    /// Role ARN for delegated authentication
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    /// Shared secret passed as the STS ExternalId.
    ///
    /// Recommended if `role_arn` is not null.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl S3ConfigIn {
    pub fn new(bucket: String) -> Self {
        Self {
            bucket,
            access_key_id: None,
            secret_access_key: None,
            region: None,
            endpoint_url: None,
            role_arn: None,
            external_id: None,
        }
    }
}
