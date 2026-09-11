// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct AutoconfigCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct AutoconfigRotateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Autoconfig<'a> {
    cfg: &'a Configuration,
}

impl<'a> Autoconfig<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create an AutoConfig subscription.
    pub async fn create(
        &self,
        app_id: String,
        options: Option<AutoconfigCreateOptions>,
    ) -> Result<AutoConfigOut> {
        let AutoconfigCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/app/{app_id}/autoconfig")
            .with_path_param("app_id", app_id)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .execute(self.cfg)
            .await
    }

    /// Rotate the auth token and signing secret for an AutoConfig subscription.
    pub async fn rotate(
        &self,
        app_id: String,
        autoconfig_id: String,
        rotate_subscription_in2: RotateSubscriptionIn2,
        options: Option<AutoconfigRotateOptions>,
    ) -> Result<AutoConfigOut> {
        let AutoconfigRotateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("autoconfig_id", autoconfig_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(rotate_subscription_in2)
        .execute(self.cfg)
        .await
    }
}
