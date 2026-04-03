// this file is @generated
use serde::{Deserialize, Serialize};

use super::eviction_policy::EvictionPolicy;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheCreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<EvictionPolicy>,
}

impl CacheCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            eviction_policy: None,
        }
    }

    pub fn with_eviction_policy(mut self, value: impl Into<Option<EvictionPolicy>>) -> Self {
        self.eviction_policy = value.into();
        self
    }
}
