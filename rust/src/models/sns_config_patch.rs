// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnsConfigPatch {
    #[serde(rename = "topicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(rename = "endpointUrl")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub endpoint_url: JsOption<String>,
}

impl SnsConfigPatch {
    pub fn new() -> Self {
        Self {
            topic_arn: None,
            region: None,
            access_key_id: None,
            secret_access_key: None,
            endpoint_url: JsOption::Undefined,
        }
    }
}

impl Default for SnsConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
