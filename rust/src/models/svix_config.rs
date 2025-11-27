// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SvixConfig {
    pub secret: String,
}

impl SvixConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
