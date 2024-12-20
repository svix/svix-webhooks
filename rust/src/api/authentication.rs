use super::PostOptions;
use crate::{apis::authentication_api, error::Result, models::*, Configuration};

pub struct Authentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> Authentication<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn dashboard_access(
        &self,
        app_id: String,
        options: Option<PostOptions>,
    ) -> Result<DashboardAccessOut> {
        let options = options.unwrap_or_default();
        authentication_api::v1_period_authentication_period_dashboard_access(
            self.cfg,
            authentication_api::V1PeriodAuthenticationPeriodDashboardAccessParams {
                app_id,
                idempotency_key: options.idempotency_key,
            },
        )
        .await
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

        authentication_api::v1_period_authentication_period_app_portal_access(
            self.cfg,
            authentication_api::V1PeriodAuthenticationPeriodAppPortalAccessParams {
                app_id,
                app_portal_access_in,
                idempotency_key,
            },
        )
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

        authentication_api::v1_period_authentication_period_expire_all(
            self.cfg,
            authentication_api::V1PeriodAuthenticationPeriodExpireAllParams {
                app_id,
                application_token_expire_in,
                idempotency_key,
            },
        )
        .await
    }

    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    pub async fn logout(&self, options: Option<PostOptions>) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        authentication_api::v1_period_authentication_period_logout(
            self.cfg,
            authentication_api::V1PeriodAuthenticationPeriodLogoutParams { idempotency_key },
        )
        .await
    }
}
