use super::PostOptions;
use crate::{error::Result, models::*, Configuration};

pub struct Authentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> Authentication<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Use this function to get magic links (and authentication codes) for
    /// connecting your users to the Consumer Application Portal.
    pub async fn app_portal_access(
        &self,
        app_id: String,
        app_portal_access_in: AppPortalAccessIn,
        options: Option<PostOptions>,
    ) -> Result<AppPortalAccessOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/app-portal-access/{app_id}",
        )
        .with_path_param("app_id", app_id)
        .with_body_param(app_portal_access_in)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .execute(self.cfg)
        .await
    }

    /// Expire all of the tokens associated with a specific application.
    pub async fn expire_all(
        &self,
        app_id: String,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: Option<PostOptions>,
    ) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/auth/app/{app_id}/expire-all")
            .with_path_param("app_id", app_id)
            .with_body_param(application_token_expire_in)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// DEPRECATED: Please use `app-portal-access` instead.
    ///
    /// Use this function to get magic links (and authentication codes) for
    /// connecting your users to the Consumer Application Portal.
    #[deprecated]
    pub async fn dashboard_access(
        &self,
        app_id: String,
        options: Option<PostOptions>,
    ) -> Result<DashboardAccessOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/dashboard-access/{app_id}",
        )
        .with_path_param("app_id", app_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .execute(self.cfg)
        .await
    }

    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    pub async fn logout(&self, options: Option<PostOptions>) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/auth/logout")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }
}
