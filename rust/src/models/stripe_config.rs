// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StripeConfig {
    pub secret: String,
}

impl StripeConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
