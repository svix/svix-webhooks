// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until at least one token is available (only present when remaining is 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_millis: Option<u64>,
}

impl RateLimitGetRemainingOut {
    pub fn new(remaining: u64) -> Self {
        Self {
            remaining,
            retry_after_millis: None,
        }
    }

    pub fn with_retry_after_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.retry_after_millis = value.into();
        self
    }
}
