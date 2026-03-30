// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CacheSetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl_ms: u64,
}

impl CacheSetIn {
    pub fn new(value: Vec<u8>, ttl_ms: u64) -> Self {
        Self {
            namespace: None,
            value,
            ttl_ms,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CacheSetIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl_ms: u64,
}
