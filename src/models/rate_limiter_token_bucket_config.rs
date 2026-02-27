// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterTokenBucketConfig {
    /// Maximum capacity of the bucket
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    pub refill_amount: u64,

    /// Interval in seconds between refills (minimum 1 second)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refill_interval: Option<u64>,
}

impl RateLimiterTokenBucketConfig {
    pub fn new(capacity: u64, refill_amount: u64) -> Self {
        Self {
            capacity,
            refill_amount,
            refill_interval: None,
        }
    }
}
