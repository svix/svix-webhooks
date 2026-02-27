// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct RateLimiter<'a> {
    cfg: &'a Configuration,
}

impl<'a> RateLimiter<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Rate Limiter Check and Consume
    pub async fn limit(
        &self,
        rate_limiter_check_in: RateLimiterCheckIn,
    ) -> Result<RateLimiterCheckOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limiter/limit")
            .with_body(rate_limiter_check_in)
            .execute(self.cfg)
            .await
    }

    /// Rate Limiter Get Remaining
    pub async fn get_remaining(
        &self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
    ) -> Result<RateLimiterGetRemainingOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limiter/get-remaining")
            .with_body(rate_limiter_get_remaining_in)
            .execute(self.cfg)
            .await
    }
}
