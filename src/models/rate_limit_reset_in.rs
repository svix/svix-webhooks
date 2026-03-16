// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_token_bucket_config::RateLimitTokenBucketConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitResetIn {
    pub key: String,

    /// Rate limiter configuration
    pub config: RateLimitTokenBucketConfig,
}

impl RateLimitResetIn {
    pub fn new(key: String, config: RateLimitTokenBucketConfig) -> Self {
        Self { key, config }
    }
}
