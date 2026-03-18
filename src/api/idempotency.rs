// this file is @generated
use super::IdempotencyNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct Idempotency<'a> {
    cfg: &'a Configuration,
}

impl<'a> Idempotency<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> IdempotencyNamespace<'a> {
        IdempotencyNamespace::new(self.cfg)
    }

    /// Start an idempotent request
    pub async fn start(
        &self,
        key: String,
        idempotency_start_in: IdempotencyStartIn,
    ) -> Result<IdempotencyStartOut> {
        let idempotency_start_in = IdempotencyStartIn_ {
            key,
            ttl: idempotency_start_in.ttl,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/start")
            .with_body(idempotency_start_in)
            .execute(self.cfg)
            .await
    }

    /// Complete an idempotent request with a response
    pub async fn complete(
        &self,
        key: String,
        idempotency_complete_in: IdempotencyCompleteIn,
    ) -> Result<IdempotencyCompleteOut> {
        let idempotency_complete_in = IdempotencyCompleteIn_ {
            key,
            response: idempotency_complete_in.response,
            ttl: idempotency_complete_in.ttl,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/complete")
            .with_body(idempotency_complete_in)
            .execute(self.cfg)
            .await
    }

    /// Abandon an idempotent request (remove lock without saving response)
    pub async fn abort(
        &self,
        key: String,
        idempotency_abort_in: IdempotencyAbortIn,
    ) -> Result<IdempotencyAbortOut> {
        let _unused = idempotency_abort_in;
        let idempotency_abort_in = IdempotencyAbortIn_ { key };

        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/abort")
            .with_body(idempotency_abort_in)
            .execute(self.cfg)
            .await
    }
}
