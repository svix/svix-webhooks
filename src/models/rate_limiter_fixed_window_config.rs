// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterFixedWindowConfig {
    /// Window size in seconds
    pub window_size: u64,

    /// Maximum number of requests allowed within the window
    pub max_requests: u64,
}

impl RateLimiterFixedWindowConfig {
    pub fn new(window_size: u64, max_requests: u64) -> Self {
        Self {
            window_size,
            max_requests,
        }
    }
}
