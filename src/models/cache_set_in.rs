// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CacheSetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Time to live in milliseconds
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}

impl CacheSetIn {
    pub fn new(ttl: std::time::Duration) -> Self {
        Self {
            namespace: None,
            ttl,
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
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}
