// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VeriffConfig {
    pub secret: String,
}

impl VeriffConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
