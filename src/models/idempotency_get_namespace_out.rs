// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyGetNamespaceOut {
    pub name: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl IdempotencyGetNamespaceOut {
    pub fn new(name: String, created: jiff::Timestamp, updated: jiff::Timestamp) -> Self {
        Self {
            name,
            created,
            updated,
        }
    }
}
