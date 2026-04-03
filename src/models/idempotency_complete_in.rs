// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyCompleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The response to cache
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,

    /// TTL in milliseconds for the cached response
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}

impl IdempotencyCompleteIn {
    pub fn new(response: Vec<u8>, ttl: std::time::Duration) -> Self {
        Self {
            namespace: None,
            response,
            ttl,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct IdempotencyCompleteIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// The response to cache
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,

    /// TTL in milliseconds for the cached response
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}
