// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct IngestSourceListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct IngestSourceCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct IngestSourceRotateTokenOptions {
    pub idempotency_key: Option<String>,
}

pub struct IngestSource<'a> {
    cfg: &'a Configuration,
}

impl<'a> IngestSource<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List of all the organization's Ingest Sources.
    pub async fn list(
        &self,
        options: Option<IngestSourceListOptions>,
    ) -> Result<ListResponseIngestSourceOut> {
        let IngestSourceListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source",
        )
        .with_optional_query_param(
            "limit", limit,
        )
        .with_optional_query_param(
            "iterator", iterator,
        )
        .with_optional_query_param(
            "order", order,
        )
        .execute(self.cfg)
        .await
    }

    /// Create Ingest Source.
    pub async fn create(
        &self,
        ingest_source_in: IngestSourceIn,
        options: Option<IngestSourceCreateOptions>,
    ) -> Result<IngestSourceOut> {
        let IngestSourceCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/ingest/api/v1/source",
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(ingest_source_in)
        .execute(self.cfg)
        .await
    }

    /// Get an Ingest Source by id or uid.
    pub async fn get(
        &self,
        source_id: String,
    ) -> Result<IngestSourceOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update an Ingest Source.
    pub async fn update(
        &self,
        source_id: String,
        ingest_source_in: IngestSourceIn,
    ) -> Result<IngestSourceOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/ingest/api/v1/source/{source_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_body_param(ingest_source_in)
        .execute(self.cfg)
        .await
    }

    /// Delete an Ingest Source.
    pub async fn delete(
        &self,
        source_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/ingest/api/v1/source/{source_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Rotate the Ingest Source's Url Token.
    ///
    /// This will rotate the ingest source's token, which is used to
    /// construct the unique `ingestUrl` for the source. Previous tokens
    /// will remain valid for 48 hours after rotation. The token can be
    /// rotated a maximum of three times within the 48-hour period.
    pub async fn rotate_token(
        &self,
        source_id: String,
        options: Option<IngestSourceRotateTokenOptions>,
    ) -> Result<RotateTokenOut> {
        let IngestSourceRotateTokenOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/ingest/api/v1/source/{source_id}/token/rotate",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .execute(self.cfg)
        .await
    }
}
