// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct IngestAuthenticationConsumerPortalAccessOptions {
    pub idempotency_key: Option<String>,
}

pub struct IngestAuthentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> IngestAuthentication<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get access to the Ingest Source Consumer Portal.
    pub async fn consumer_portal_access(
        &self,
        source_id: String,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: Option<IngestAuthenticationConsumerPortalAccessOptions>,
    ) -> Result<AppPortalAccessOut> {
        let IngestAuthenticationConsumerPortalAccessOptions { idempotency_key } =
            options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::POST,
            "/ingest/api/v1/source/{source_id}/dashboard",
        )
        .with_path_param("source_id", source_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(ingest_source_consumer_portal_access_in)
        .execute(self.cfg)
        .await
    }
}
