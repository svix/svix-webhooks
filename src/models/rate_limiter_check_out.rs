// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_status::RateLimitStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterCheckOut {
    /// Whether the request is allowed
    pub status: RateLimitStatus,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl RateLimiterCheckOut {
    pub fn new(status: RateLimitStatus, remaining: u64) -> Self {
        Self {
            status,
            remaining,
            retry_after: None,
        }
    }

    pub fn with_retry_after(mut self, value: impl Into<Option<u64>>) -> Self {
        self.retry_after = value.into();
        self
    }
}
