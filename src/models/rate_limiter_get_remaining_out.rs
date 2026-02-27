// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until at least one token is available (only present when remaining is 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl RateLimiterGetRemainingOut {
    pub fn new(remaining: u64) -> Self {
        Self {
            remaining,
            retry_after: None,
        }
    }
}
