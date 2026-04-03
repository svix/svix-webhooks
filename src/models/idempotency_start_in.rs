// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyStartIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// TTL in milliseconds for the lock/response
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}

impl IdempotencyStartIn {
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
pub(crate) struct IdempotencyStartIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// TTL in milliseconds for the lock/response
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}
