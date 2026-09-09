// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct DestinationTransformation<'a> {
    cfg: &'a Configuration,
}

impl<'a> DestinationTransformation<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get the transformation code associated with this destination.
    pub async fn get(
        &self,
        app_id: String,
        destination_id: String,
    ) -> Result<DestinationTransformationOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this destination.
    pub async fn patch(
        &self,
        app_id: String,
        destination_id: String,
        destination_transform_in: DestinationTransformIn,
    ) -> Result<EmptyResponse> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .with_body_param(destination_transform_in)
        .execute(self.cfg)
        .await
    }
}
