// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyCompleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The response to cache
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,

    /// Optional metadata to store alongside the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,

    /// How long to keep the idempotency response for.
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}

impl IdempotencyCompleteIn {
    pub fn new(response: Vec<u8>, ttl: std::time::Duration) -> Self {
        Self {
            namespace: None,
            response,
            context: None,
            ttl,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_context(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.context = value.into();
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

    /// Optional metadata to store alongside the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,

    /// How long to keep the idempotency response for.
    #[serde(rename = "ttl_ms", with = "crate::duration_ms_serde")]
    pub ttl: std::time::Duration,
}
