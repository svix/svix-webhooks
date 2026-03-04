// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvCreateNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub storage_type: StorageType,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl KvCreateNamespaceOut {
    pub fn new(
        name: String,
        storage_type: StorageType,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            storage_type,
            created,
            updated,
        }
    }
}
