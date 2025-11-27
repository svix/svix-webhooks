// this file is @generated
use super::{
    IngestEndpoint,
    IngestSource,
};
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct IngestDashboardOptions {
    pub idempotency_key: Option<String>,
}

pub struct Ingest<'a> {
    cfg: &'a Configuration,
}

impl<'a> Ingest<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    pub fn endpoint(&self) -> IngestEndpoint<'a> {
        IngestEndpoint::new(self.cfg)
    }

    pub fn source(&self) -> IngestSource<'a> {
        IngestSource::new(self.cfg)
    }

    /// Get access to the Ingest Source Consumer Portal.
    pub async fn dashboard(
        &self,
        source_id: String,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: Option<IngestDashboardOptions>,
    ) -> Result<DashboardAccessOut> {
        let IngestDashboardOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/ingest/api/v1/source/{source_id}/dashboard",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(ingest_source_consumer_portal_access_in)
        .execute(self.cfg)
        .await
    }
}
