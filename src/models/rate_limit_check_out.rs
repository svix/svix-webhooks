// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCheckOut {
    /// Whether the request is allowed
    pub allowed: bool,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until enough tokens are available (only present when allowed is false)
    #[serde(
        rename = "retry_after_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub retry_after: Option<std::time::Duration>,
}

impl RateLimitCheckOut {
    pub fn new(allowed: bool, remaining: u64) -> Self {
        Self {
            allowed,
            remaining,
            retry_after: None,
        }
    }

    pub fn with_retry_after(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.retry_after = value.into();
        self
    }
}
