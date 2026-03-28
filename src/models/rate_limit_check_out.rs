// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCheckOut {
    /// Whether the request is allowed
    pub allowed: bool,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
}

impl RateLimitCheckOut {
    pub fn new(allowed: bool, remaining: u64) -> Self {
        Self {
            allowed,
            remaining,
            retry_after_ms: None,
        }
    }

    pub fn with_retry_after_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.retry_after_ms = value.into();
        self
    }
}
