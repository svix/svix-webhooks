// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for an SQS sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SqsConfigIn {
    #[serde(rename = "queueUrl")]
    pub queue_url: String,

    /// The region of the SQS queue.
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

impl SqsConfigIn {
    pub fn new(queue_url: String) -> Self {
        Self {
            queue_url,
            region: None,
            access_key_id: None,
            secret_access_key: None,
            endpoint_url: None,
        }
    }
}
