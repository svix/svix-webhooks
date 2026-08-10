// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct S3ConfigOut {
    pub bucket: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    pub region: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}
