// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OrumIoConfig {
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl OrumIoConfig {
    pub fn new(public_key: String) -> Self {
        Self { public_key }
    }
}
