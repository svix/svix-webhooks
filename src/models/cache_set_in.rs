// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CacheSetIn {
    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl: u64,
}

impl CacheSetIn {
    pub fn new(value: Vec<u8>, ttl: u64) -> Self {
        Self { value, ttl }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CacheSetIn_ {
    pub key: String,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl: u64,
}
