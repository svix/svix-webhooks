// this file is @generated
use super::{AutoconfigDestination, AutoconfigEndpoint};
use crate::{error::Result, models::*, Configuration};

pub struct Autoconfig<'a> {
    cfg: &'a Configuration,
}

impl<'a> Autoconfig<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn destination(&self) -> AutoconfigDestination<'a> {
        AutoconfigDestination::new(self.cfg)
    }

    pub fn endpoint(&self) -> AutoconfigEndpoint<'a> {
        AutoconfigEndpoint::new(self.cfg)
    }

    /// Get an AutoConfig subscription, including the bound endpoint or
    /// destination if any.
    pub async fn get(
        &self,
        app_id: String,
        autoconfig_id: String,
    ) -> Result<AutoConfigSubscriptionOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("autoconfig_id", autoconfig_id)
        .execute(self.cfg)
        .await
    }
}
