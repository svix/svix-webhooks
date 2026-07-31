// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct IngestEndpointTransformation<'a> {
    cfg: &'a Configuration,
}

impl<'a> IngestEndpointTransformation<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get the transformation code associated with this ingest endpoint.
    pub async fn transformation(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<IngestEndpointTransformationOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param("source_id", source_id)
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this ingest
    /// endpoint.
    pub async fn patch(
        &self,
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_transformation_patch: IngestEndpointTransformationPatch,
    ) -> Result<()> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param("source_id", source_id)
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(ingest_endpoint_transformation_patch)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
