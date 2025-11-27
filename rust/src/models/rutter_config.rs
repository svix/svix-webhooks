// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RutterConfig {
    pub secret: String,
}

impl RutterConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
