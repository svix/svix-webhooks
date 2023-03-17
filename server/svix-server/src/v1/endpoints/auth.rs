use aide::{
    axum::{routing::post_with, ApiRouter},
    transform::TransformOperation,
};
use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use svix_server_derive::aide_annotate;
use validator::Validate;

use crate::{
    core::{permissions, security::generate_app_token, types::FeatureFlagSet},
    error::{HttpError, Result},
    v1::utils::{api_not_implemented, openapi_tag, ValidatedJson},
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

/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
#[aide_annotate(op_id = "get_app_portal_access_api_v1_auth_app_portal_access__app_id___post")]
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

/// DEPRECATED: Please use `app-portal-access` instead.
///
/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
#[aide_annotate(op_id = "get_dashboard_access_api_v1_auth_dashboard_access__app_id___post")]
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

fn logout_operation(op: TransformOperation) -> TransformOperation {
    op.id("logout_api_v1_auth_logout__post")
        .summary("Logout")
        .description(LOGOUT_DESCRIPTION)
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Authentication");
    ApiRouter::new()
        .api_route_with(
            "/auth/dashboard-access/:app_id/",
            post_with(dashboard_access, dashboard_access_operation),
            &tag,
        )
        .api_route_with(
            "/auth/logout/",
            post_with(api_not_implemented, logout_operation),
            &tag,
        )
        .api_route_with(
            "/auth/app-portal-access/:app_id/",
            post_with(app_portal_access, app_portal_access_operation),
            tag,
        )
}
