// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnsConfigOut {
    #[serde(rename = "topicArn")]
    pub topic_arn: String,

    pub region: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
}
