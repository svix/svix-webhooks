// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitGetNamespaceOut {
    pub name: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl RateLimitGetNamespaceOut {
    pub fn new(name: String, created: jiff::Timestamp, updated: jiff::Timestamp) -> Self {
        Self {
            name,
            created,
            updated,
        }
    }
}
