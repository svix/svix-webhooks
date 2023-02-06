use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    core::{permissions, security::generate_app_token, types::FeatureFlagSet},
    error::{HttpError, Result},
    v1::utils::{api_not_implemented, openapi_desc, openapi_tag, ValidatedJson},
    AppState,
};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DashboardAccessOut {
    pub url: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppPortalAccessIn {
    /// The set of feature flags the created token will have access to.
    #[serde(default, skip_serializing_if = "FeatureFlagSet::is_empty")]
    pub feature_flags: FeatureFlagSet,
}

pub type AppPortalAccessOut = DashboardAccessOut;

const APP_PORTAL_ACCESS_DESCRIPTION: &str = "Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.";

async fn app_portal_access(
    State(AppState { cfg, .. }): State<AppState>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<AppPortalAccessIn>,
) -> Result<Json<AppPortalAccessOut>> {
    let token = generate_app_token(
        &cfg.jwt_secret,
        app.org_id,
        app.id.clone(),
        data.feature_flags,
    )?;

    let login_key = serde_json::to_vec(&serde_json::json!({
        "appId": app.id,
        "token": token,
        "region": &cfg.internal.region,
    }))
    .map_err(|_| HttpError::internal_server_error(None, None))?;

    let login_key = base64::encode(login_key);

    // Included for API compatibility, but this URL will not be useful
    let url = format!("{}/login#key={}", &cfg.internal.app_portal_url, login_key);

    Ok(Json(DashboardAccessOut { url, token }))
}

const DASHBOARD_ACCESS_DESCRIPTION: &str = r#"
DEPRECATED: Please use `app-portal-access` instead.

Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
"#;

async fn dashboard_access(
    state: State<AppState>,
    permissions: permissions::OrganizationWithApplication,
) -> Result<Json<DashboardAccessOut>> {
    app_portal_access(
        state,
        permissions,
        ValidatedJson(AppPortalAccessIn {
            feature_flags: FeatureFlagSet::default(),
        }),
    )
    .await
}

const LOGOUT_DESCRIPTION: &str = r#"
Logout an app token.

Trying to log out other tokens will fail.
"#;

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Authentication");
    ApiRouter::new()
        .api_route_with(
            "/auth/dashboard-access/:app_id/",
            post_with(dashboard_access, openapi_desc(DASHBOARD_ACCESS_DESCRIPTION)),
            &tag,
        )
        .api_route_with(
            "/auth/logout/",
            post_with(api_not_implemented, openapi_desc(LOGOUT_DESCRIPTION)),
            &tag,
        )
        .api_route_with(
            "/auth/app-portal-access/:app_id/",
            post_with(
                app_portal_access,
                openapi_desc(APP_PORTAL_ACCESS_DESCRIPTION),
            ),
            tag,
        )
}
