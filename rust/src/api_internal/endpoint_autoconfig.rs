// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct EndpointAutoconfig<'a> {
    cfg: &'a Configuration,
}

impl<'a> EndpointAutoconfig<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or update the HTTP endpoint for an AutoConfig subscription.
    pub async fn subscribe(
        &self,
        app_id: String,
        autoconfig_id: String,
        endpoint_in: EndpointIn,
    ) -> Result<EndpointOut> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("autoconfig_id", autoconfig_id)
        .with_body_param(endpoint_in)
        .execute(self.cfg)
        .await
    }
}
