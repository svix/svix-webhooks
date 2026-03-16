// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitTokenBucketConfig {
    /// Maximum capacity of the bucket
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    pub refill_amount: u64,

    /// Interval in milliseconds between refills (minimum 1 millisecond)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refill_interval_millis: Option<u64>,
}

impl RateLimitTokenBucketConfig {
    pub fn new(capacity: u64, refill_amount: u64) -> Self {
        Self {
            capacity,
            refill_amount,
            refill_interval_millis: None,
        }
    }

    pub fn with_refill_interval_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.refill_interval_millis = value.into();
        self
    }
}
