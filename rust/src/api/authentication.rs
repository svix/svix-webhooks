// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct AuthenticationAppPortalAccessOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct AuthenticationExpireAllOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct AuthenticationLogoutOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct AuthenticationStreamPortalAccessOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct AuthenticationRotateStreamPollerTokenOptions {
    pub idempotency_key: Option<String>,
}

pub struct Authentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> Authentication<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// Use this function to get magic links (and authentication codes) for
    /// connecting your users to the Consumer Application Portal.
    pub async fn app_portal_access(
        &self,
        app_id: String,
        app_portal_access_in: AppPortalAccessIn,
        options: Option<AuthenticationAppPortalAccessOptions>,
    ) -> Result<AppPortalAccessOut> {
        let AuthenticationAppPortalAccessOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/app-portal-access/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(app_portal_access_in)
        .execute(self.cfg)
        .await
    }

    /// Expire all of the tokens associated with a specific application.
    pub async fn expire_all(
        &self,
        app_id: String,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: Option<AuthenticationExpireAllOptions>,
    ) -> Result<()> {
        let AuthenticationExpireAllOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/app/{app_id}/expire-all",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(application_token_expire_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    #[deprecated = "Please use app_portal_access instead."]
    #[allow(deprecated)]
    pub async fn dashboard_access(
        &self,
        app_id: String,
        options: Option<super::AuthenticationDashboardAccessOptions>,
    ) -> Result<DashboardAccessOut> {
        let super::AuthenticationDashboardAccessOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/dashboard-access/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .execute(self.cfg)
        .await
    }

    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    pub async fn logout(
        &self,
        options: Option<AuthenticationLogoutOptions>,
    ) -> Result<()> {
        let AuthenticationLogoutOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/logout",
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Use this function to get magic links (and authentication codes) for
    /// connecting your users to the Stream Consumer Portal.
    pub async fn stream_portal_access(
        &self,
        stream_id: String,
        stream_portal_access_in: StreamPortalAccessIn,
        options: Option<AuthenticationStreamPortalAccessOptions>,
    ) -> Result<AppPortalAccessOut> {
        let AuthenticationStreamPortalAccessOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/stream-portal-access/{stream_id}",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(stream_portal_access_in)
        .execute(self.cfg)
        .await
    }

    /// Get the current auth token for the stream poller.
    pub async fn get_stream_poller_token(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<ApiTokenOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Create a new auth token for the stream poller API.
    pub async fn rotate_stream_poller_token(
        &self,
        stream_id: String,
        sink_id: String,
        rotate_poller_token_in: RotatePollerTokenIn,
        options: Option<AuthenticationRotateStreamPollerTokenOptions>,
    ) -> Result<ApiTokenOut> {
        let AuthenticationRotateStreamPollerTokenOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token/rotate",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(rotate_poller_token_in)
        .execute(self.cfg)
        .await
    }
}
