// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DocusignConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl DocusignConfig {
    pub fn new() -> Self {
        Self { secret: None }
    }
}

impl Default for DocusignConfig {
    fn default() -> Self {
        Self::new()
    }
}
