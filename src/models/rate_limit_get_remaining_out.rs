// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until at least one token is available (only present when remaining is 0)
    #[serde(
        rename = "retry_after_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub retry_after: Option<std::time::Duration>,
}

impl RateLimitGetRemainingOut {
    pub fn new(remaining: u64) -> Self {
        Self {
            remaining,
            retry_after: None,
        }
    }

    pub fn with_retry_after(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.retry_after = value.into();
        self
    }
}
