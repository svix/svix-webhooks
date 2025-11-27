// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ZoomConfig {
    pub secret: String,
}

impl ZoomConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
