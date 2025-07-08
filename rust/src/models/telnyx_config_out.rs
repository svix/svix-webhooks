// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelnyxConfigOut {
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl TelnyxConfigOut {
    pub fn new(public_key: String) -> Self {
        Self { public_key }
    }
}
