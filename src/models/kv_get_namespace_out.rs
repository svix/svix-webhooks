// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub storage_type: StorageType,

    pub created_at: jiff::Timestamp,

    pub updated_at: jiff::Timestamp,
}

impl KvGetNamespaceOut {
    pub fn new(
        name: String,
        storage_type: StorageType,
        created_at: jiff::Timestamp,
        updated_at: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            storage_type,
            created_at,
            updated_at,
        }
    }
}
