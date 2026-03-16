// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_token_bucket_config::RateLimitTokenBucketConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitGetRemainingIn {
    pub key: String,

    /// Rate limiter configuration
    pub config: RateLimitTokenBucketConfig,
}

impl RateLimitGetRemainingIn {
    pub fn new(key: String, config: RateLimitTokenBucketConfig) -> Self {
        Self { key, config }
    }
}
