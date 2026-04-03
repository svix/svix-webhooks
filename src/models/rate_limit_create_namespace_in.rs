// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCreateNamespaceIn {
    pub name: String,
}

impl RateLimitCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
