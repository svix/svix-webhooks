// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3ConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

impl S3ConfigPatch {
    pub fn new() -> Self {
        Self {
            bucket: None,
            access_key_id: None,
            secret_access_key: None,
            role_arn: None,
            external_id: None,
            region: None,
            endpoint_url: None,
        }
    }
}

impl Default for S3ConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
