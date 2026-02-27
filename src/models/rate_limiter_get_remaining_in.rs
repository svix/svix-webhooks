// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    rate_limiter_fixed_window_config::RateLimiterFixedWindowConfig,
    rate_limiter_token_bucket_config::RateLimiterTokenBucketConfig,
};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterGetRemainingIn {
    pub key: String,

    #[serde(flatten)]
    pub config: RateLimiterGetRemainingInConfig,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "method", content = "config")]
pub enum RateLimiterGetRemainingInConfig {
    #[serde(rename = "token_bucket")]
    TokenBucket(RateLimiterTokenBucketConfig),
    #[serde(rename = "fixed_window")]
    FixedWindow(RateLimiterFixedWindowConfig),
}
