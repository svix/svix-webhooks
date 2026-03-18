// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyCompleteIn {
    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    pub ttl: u64,
}

impl IdempotencyCompleteIn {
    pub fn new(response: Vec<u8>, ttl: u64) -> Self {
        Self { response, ttl }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct IdempotencyCompleteIn_ {
    pub key: String,

    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    pub ttl: u64,
}
