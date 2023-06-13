use aide::{
    axum::{routing::post_with, ApiRouter},
    transform::TransformOperation,
};
use axum::{
    extract::{Path, State},
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use svix_server_derive::aide_annotate;
use validator::Validate;

use crate::{
    core::{
        cache::{string_kv_def, CacheBehavior, CacheKey, StringCacheValue},
        permissions,
        security::generate_app_token,
        types::FeatureFlagSet,
    },
    ctx, err_generic,
    error::{HttpError, Result},
    v1::utils::{api_not_implemented, openapi_tag, ApplicationPath, ValidatedJson},
    AppState,
};

fn login_url_example() -> &'static str {
    "https://app.svix.com/login#key=eyJhcHBJZCI6ICJhcHBfMXRSdFl"
}

fn token_example() -> &'static str {
    "appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O"
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DashboardAccessOut {
    #[schemars(url, example = "login_url_example", length(min = 1, max = 65_536))]
    pub url: String,
    #[schemars(example = "token_example")]
    pub token: String,
}

fn feature_flag_set_example() -> FeatureFlagSet {
    FeatureFlagSet::new()
}

#[derive(Deserialize, Serialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppPortalAccessIn {
    /// The set of feature flags the created token will have access to.
    #[serde(default, skip_serializing_if = "FeatureFlagSet::is_empty")]
    #[schemars(example = "feature_flag_set_example")]
    pub feature_flags: FeatureFlagSet,
}

#[derive(Serialize, JsonSchema)]
pub struct AppPortalAccessOut {
    #[serde(flatten)]
    common_: DashboardAccessOut,
}

impl From<DashboardAccessOut> for AppPortalAccessOut {
    fn from(common_: DashboardAccessOut) -> Self {
        Self { common_ }
    }
}

/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
#[aide_annotate(op_id = "v1.authentication.app-portal-access")]
async fn app_portal_access(
    State(AppState { cfg, ref cache, .. }): State<AppState>,
    _: Path<ApplicationPath>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<AppPortalAccessIn>,
) -> Result<Json<AppPortalAccessOut>> {
    let one_time_token = generate_one_time_token()?;
    let key = OneTimeTokenCacheKey::new(&one_time_token);

    // The "exchange" endpoint will give this JWT back when requested with the one time token.
    let token = generate_app_token(
        &cfg.jwt_secret,
        app.org_id,
        app.id.clone(),
        data.feature_flags,
    )?;
    ctx!(
        cache
            .set_string(&key, &token, Duration::from_secs(60 * 60 * 24 * 7))
            .await
    )?;
    let login_key = serde_json::to_vec(&serde_json::json!({
        "appId": app.id,
        "oneTimeToken": one_time_token,
        "region": &cfg.internal.region,
    }))
    .map_err(|_| HttpError::internal_server_error(None, None))?;

    let login_key = base64::encode(login_key);

    // Included for API compatibility, but this URL will not be useful
    let url = format!("{}/login#key={}", &cfg.internal.app_portal_url, login_key);

    Ok(Json(AppPortalAccessOut::from(DashboardAccessOut {
        url,
        token,
    })))
}

/// DEPRECATED: Please use `app-portal-access` instead.
///
/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
#[aide_annotate(op_id = "v1.authentication.dashboard-access")]
async fn dashboard_access(
    state: State<AppState>,
    path: Path<ApplicationPath>,
    permissions: permissions::OrganizationWithApplication,
) -> Result<Json<DashboardAccessOut>> {
    app_portal_access(
        state,
        path,
        permissions,
        ValidatedJson(AppPortalAccessIn {
            feature_flags: FeatureFlagSet::default(),
        }),
    )
    .await
    .map(|Json(AppPortalAccessOut { common_: out })| Json(out))
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

type OneTimeTokenCacheValue = String;
string_kv_def!(
    OneTimeTokenCacheKey,
    OneTimeTokenCacheValue,
    "AUTH_ONE_TIME_TOKEN"
);

impl OneTimeTokenCacheKey {
    pub fn new(one_time_token: &str) -> OneTimeTokenCacheKey {
        OneTimeTokenCacheKey(format!("{}_{}", Self::PREFIX_CACHE, one_time_token))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenIn {
    pub one_time_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenOut {
    pub token: String,
}

/// This is a one time token
#[aide_annotate(op_id = "v1.authentication.exchange-one-time-token")]
async fn exchange_one_time_token(
    State(AppState { ref cache, .. }): State<AppState>,
    Json(OneTimeTokenIn { one_time_token }): Json<OneTimeTokenIn>,
) -> Result<Json<OneTimeTokenOut>> {
    let key = OneTimeTokenCacheKey::new(&one_time_token);
    let token = ctx!(cache.get_string(&key).await)?.ok_or_else(|| {
        HttpError::unauthorized(
            None,
            Some("One time token not found. Has it already been used?".to_owned()),
        )
    })?;
    ctx!(cache.delete(&key).await)?;

    Ok(Json(OneTimeTokenOut { token }))
}

fn generate_one_time_token() -> std::result::Result<String, crate::error::Error> {
    const KEY_SIZE: usize = 24;
    let mut buf = [0u8; KEY_SIZE];
    ctx!(getrandom::getrandom(&mut buf).map_err(|e| err_generic!("getrandom failure: {}", e)))?;
    Ok(base64::encode_config(buf, base64::URL_SAFE_NO_PAD))
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

pub fn authless_router(hide_secret_routes: bool) -> ApiRouter<AppState> {
    let tag = openapi_tag("Authentication");
    ApiRouter::new().api_route_with(
        "/auth/one-time-token/",
        post_with(exchange_one_time_token, exchange_one_time_token_operation),
        |op| op.hidden(hide_secret_routes).with(&tag),
    )
}
