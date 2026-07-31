// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct EndpointTransformation<'a> {
    cfg: &'a Configuration,
}

impl<'a> EndpointTransformation<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get the transformation code associated with this endpoint.
    pub async fn get(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointTransformationOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this endpoint.
    pub async fn patch(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_patch: EndpointTransformationPatch,
    ) -> Result<()> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(endpoint_transformation_patch)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
