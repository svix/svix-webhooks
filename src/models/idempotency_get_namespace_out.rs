// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyGetNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub created_at: jiff::Timestamp,

    pub updated_at: jiff::Timestamp,
}

impl IdempotencyGetNamespaceOut {
    pub fn new(name: String, created_at: jiff::Timestamp, updated_at: jiff::Timestamp) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            created_at,
            updated_at,
        }
    }
}
