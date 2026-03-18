// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyStartIn {
    /// TTL in seconds for the lock/response
    pub ttl: u64,
}

impl IdempotencyStartIn {
    pub fn new(ttl: u64) -> Self {
        Self { ttl }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct IdempotencyStartIn_ {
    pub key: String,

    /// TTL in seconds for the lock/response
    pub ttl: u64,
}
