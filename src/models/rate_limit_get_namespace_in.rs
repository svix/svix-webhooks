// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitGetNamespaceIn {
    pub name: String,
}

impl RateLimitGetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
