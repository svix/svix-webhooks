// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyCreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,
}

impl IdempotencyCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            max_storage_bytes: None,
        }
    }

    pub fn with_max_storage_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.max_storage_bytes = value.into();
        self
    }
}
