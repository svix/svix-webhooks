// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterFixedWindowConfig {
    /// Maximum number of requests allowed within the window
    pub max_requests: u64,

    /// Window size in seconds
    pub window_size: u64,
}

impl RateLimiterFixedWindowConfig {
    pub fn new(max_requests: u64, window_size: u64) -> Self {
        Self {
            max_requests,
            window_size,
        }
    }
}
