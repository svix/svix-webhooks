// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ShopifyConfig {
    pub secret: String,
}

impl ShopifyConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}
