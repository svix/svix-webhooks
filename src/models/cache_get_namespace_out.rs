// this file is @generated
use serde::{Deserialize, Serialize};

use super::{eviction_policy::EvictionPolicy, storage_type::StorageType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub storage_type: StorageType,

    pub eviction_policy: EvictionPolicy,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl CacheGetNamespaceOut {
    pub fn new(
        name: String,
        storage_type: StorageType,
        eviction_policy: EvictionPolicy,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            storage_type,
            eviction_policy,
            created,
            updated,
        }
    }
}
