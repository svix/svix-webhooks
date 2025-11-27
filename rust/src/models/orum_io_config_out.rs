// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OrumIoConfigOut {
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl OrumIoConfigOut {
    pub fn new(public_key: String) -> Self {
        Self {
            public_key,
        }
    }
}
