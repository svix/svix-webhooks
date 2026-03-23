// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,
}

impl AuthTokenCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            storage_type: None,
            max_storage_bytes: None,
        }
    }

    pub fn with_storage_type(mut self, value: impl Into<Option<StorageType>>) -> Self {
        self.storage_type = value.into();
        self
    }

    pub fn with_max_storage_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.max_storage_bytes = value.into();
        self
    }
}
