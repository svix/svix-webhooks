// this file is @generated
use serde::{Deserialize, Serialize};

use super::eviction_policy::EvictionPolicy;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheCreateNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub eviction_policy: EvictionPolicy,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl CacheCreateNamespaceOut {
    pub fn new(
        name: String,
        eviction_policy: EvictionPolicy,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            eviction_policy,
            created,
            updated,
        }
    }

    pub fn with_max_storage_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.max_storage_bytes = value.into();
        self
    }
}
