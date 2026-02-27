// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyGetNamespaceOut {
    pub created_at: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub name: String,

    pub updated_at: jiff::Timestamp,
}

impl IdempotencyGetNamespaceOut {
    pub fn new(created_at: jiff::Timestamp, name: String, updated_at: jiff::Timestamp) -> Self {
        Self {
            created_at,
            max_storage_bytes: None,
            name,
            updated_at,
        }
    }
}
