// this file is @generated
use super::RateLimitNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct RateLimit<'a> {
    cfg: &'a Configuration,
}

impl<'a> RateLimit<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> RateLimitNamespace<'a> {
        RateLimitNamespace::new(self.cfg)
    }

    /// Rate Limiter Check and Consume
    pub async fn limit(&self, rate_limit_check_in: RateLimitCheckIn) -> Result<RateLimitCheckOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/limit")
            .with_body(rate_limit_check_in)
            .execute(self.cfg)
            .await
    }

    /// Rate Limiter Get Remaining
    pub async fn get_remaining(
        &self,
        rate_limit_get_remaining_in: RateLimitGetRemainingIn,
    ) -> Result<RateLimitGetRemainingOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/get-remaining")
            .with_body(rate_limit_get_remaining_in)
            .execute(self.cfg)
            .await
    }

    /// Rate Limiter Reset
    pub async fn reset(&self, rate_limit_reset_in: RateLimitResetIn) -> Result<RateLimitResetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/reset")
            .with_body(rate_limit_reset_in)
            .execute(self.cfg)
            .await
    }
}
