// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckbookConfig {
    pub secret: String,
}

impl CheckbookConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
