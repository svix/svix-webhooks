// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    rate_limiter_fixed_window_config::RateLimiterFixedWindowConfig,
    rate_limiter_token_bucket_config::RateLimiterTokenBucketConfig,
};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterCheckIn {
    pub key: String,

    /// Number of tokens to consume (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u64>,

    #[serde(flatten)]
    pub config: RateLimiterCheckInConfig,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "method", content = "config")]
pub enum RateLimiterCheckInConfig {
    #[serde(rename = "token_bucket")]
    TokenBucket(RateLimiterTokenBucketConfig),
    #[serde(rename = "fixed_window")]
    FixedWindow(RateLimiterFixedWindowConfig),
}
