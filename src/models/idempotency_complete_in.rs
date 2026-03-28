// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyCompleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    pub ttl: u64,
}

impl IdempotencyCompleteIn {
    pub fn new(response: Vec<u8>, ttl: u64) -> Self {
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
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    pub ttl: u64,
}
