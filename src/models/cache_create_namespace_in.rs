// this file is @generated
use serde::{Deserialize, Serialize};

use super::{eviction_policy::EvictionPolicy, storage_type::StorageType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheCreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<EvictionPolicy>,
}

impl CacheCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            storage_type: None,
            max_storage_bytes: None,
            eviction_policy: None,
        }
    }
}
