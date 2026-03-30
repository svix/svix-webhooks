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
            namespace: idempotency_start_in.namespace,
            key,
            ttl_ms: idempotency_start_in.ttl_ms,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.idempotency.start")
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
            namespace: idempotency_complete_in.namespace,
            key,
            response: idempotency_complete_in.response,
            ttl_ms: idempotency_complete_in.ttl_ms,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.idempotency.complete")
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
        let idempotency_abort_in = IdempotencyAbortIn_ {
            namespace: idempotency_abort_in.namespace,
            key,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.idempotency.abort")
            .with_body(idempotency_abort_in)
            .execute(self.cfg)
            .await
    }
}
