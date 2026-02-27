// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheSetIn {
    pub key: String,

    /// Time to live in milliseconds
    pub ttl: u64,

    pub value: Vec<u8>,
}

impl CacheSetIn {
    pub fn new(key: String, ttl: u64, value: Vec<u8>) -> Self {
        Self { key, ttl, value }
    }
}
