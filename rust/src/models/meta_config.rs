// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetaConfig {
    pub secret: String,

    #[serde(rename = "verifyToken")]
    pub verify_token: String,
}

impl MetaConfig {
    pub fn new(secret: String, verify_token: String) -> Self {
        Self {
            secret,
            verify_token,
        }
    }
}
