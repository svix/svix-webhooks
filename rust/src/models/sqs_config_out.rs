// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SqsConfigOut {
    #[serde(rename = "queueUrl")]
    pub queue_url: String,

    pub region: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}
