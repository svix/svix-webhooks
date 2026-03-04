// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyCreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,
}

impl IdempotencyCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            storage_type: None,
            max_storage_bytes: None,
        }
    }
}
