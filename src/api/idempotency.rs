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
