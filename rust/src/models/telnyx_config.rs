// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelnyxConfig {
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl TelnyxConfig {
    pub fn new(public_key: String) -> Self {
        Self {
            public_key,
        }
    }
}
