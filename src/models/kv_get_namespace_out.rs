// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetNamespaceOut {
    pub created_at: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub name: String,

    pub storage_type: StorageType,

    pub updated_at: jiff::Timestamp,
}

impl KvGetNamespaceOut {
    pub fn new(
        created_at: jiff::Timestamp,
        name: String,
        storage_type: StorageType,
        updated_at: jiff::Timestamp,
    ) -> Self {
        Self {
            created_at,
            max_storage_bytes: None,
            name,
            storage_type,
            updated_at,
        }
    }
}
