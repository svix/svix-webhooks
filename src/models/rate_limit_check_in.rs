// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_token_bucket_config::RateLimitTokenBucketConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCheckIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// Number of tokens to consume (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u64>,

    /// Rate limiter configuration
    pub config: RateLimitTokenBucketConfig,
}

impl RateLimitCheckIn {
    pub fn new(key: String, config: RateLimitTokenBucketConfig) -> Self {
        Self {
            namespace: None,
            key,
            tokens: None,
            config,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_tokens(mut self, value: impl Into<Option<u64>>) -> Self {
        self.tokens = value.into();
        self
    }
}
