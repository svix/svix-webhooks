// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3ConfigOut {
    pub bucket: String,

    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    pub region: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,

    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}
