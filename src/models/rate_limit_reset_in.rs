// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_config::RateLimitConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitResetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// Rate limiter configuration
    pub config: RateLimitConfig,
}

impl RateLimitResetIn {
    pub fn new(key: String, config: RateLimitConfig) -> Self {
        Self {
            namespace: None,
            key,
            config,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}
