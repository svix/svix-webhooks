// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct S3Config {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    pub bucket: String,

    pub region: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
}

impl S3Config {
    pub fn new(
        access_key_id: String,
        bucket: String,
        region: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            access_key_id,
            bucket,
            region,
            secret_access_key,
        }
    }
}
