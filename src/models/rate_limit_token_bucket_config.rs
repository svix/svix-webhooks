// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitTokenBucketConfig {
    /// Maximum capacity of the bucket
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    pub refill_amount: u64,

    /// Interval in milliseconds between refills (minimum 1 millisecond)
    #[serde(
        rename = "refill_interval_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub refill_interval: Option<std::time::Duration>,
}

impl RateLimitTokenBucketConfig {
    pub fn new(capacity: u64, refill_amount: u64) -> Self {
        Self {
            capacity,
            refill_amount,
            refill_interval: None,
        }
    }

    pub fn with_refill_interval(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.refill_interval = value.into();
        self
    }
}
