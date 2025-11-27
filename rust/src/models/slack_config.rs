// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlackConfig {
    pub secret: String,
}

impl SlackConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
