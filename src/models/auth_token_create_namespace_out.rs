// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateNamespaceOut {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<u64>,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl AuthTokenCreateNamespaceOut {
    pub fn new(name: String, created: jiff::Timestamp, updated: jiff::Timestamp) -> Self {
        Self {
            name,
            max_storage_bytes: None,
            created,
            updated,
        }
    }

    pub fn with_max_storage_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.max_storage_bytes = value.into();
        self
    }
}
