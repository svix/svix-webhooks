// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct DestinationAutoconfig<'a> {
    cfg: &'a Configuration,
}

impl<'a> DestinationAutoconfig<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or update the destination for an AutoConfig subscription.
    pub async fn subscribe(
        &self,
        app_id: String,
        autoconfig_id: String,
        destination_in: DestinationIn,
    ) -> Result<DestinationOut> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("autoconfig_id", autoconfig_id)
        .with_body_param(destination_in)
        .execute(self.cfg)
        .await
    }
}
