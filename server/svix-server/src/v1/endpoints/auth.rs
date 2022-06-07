use axum::{extract::Path, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    cfg::Configuration,
    core::{
        security::{generate_token, AuthenticatedOrganization},
        types::ApplicationIdOrUid,
    },
    error::{HttpError, Result},
    v1::utils::api_not_implemented,
};

#[derive(Deserialize, Serialize)]
struct DashboardAccessOut {
    url: String,
    token: String,
}

async fn dashboard_access(
    Extension(cfg): Extension<Configuration>,
    Path(_app_id): Path<ApplicationIdOrUid>,
    AuthenticatedOrganization { permissions, app }: AuthenticatedOrganization,
) -> Result<Json<DashboardAccessOut>> {
    let token = generate_token(&cfg.jwt_secret, permissions.org_id, Some(app.id.clone()))?;

    let login_key = serde_json::to_vec(&serde_json::json!({
        "appId": app.id,
        "token": token,
        "region": "eu"
    }))
    .map_err(|_| HttpError::internal_server_errer(None, None))?;

    let login_key = base64::encode(&login_key);

    let url = format!("https://app.svix.com/login#key={}", login_key);

    Ok(Json(DashboardAccessOut { url, token }))
}

pub fn router() -> Router {
    Router::new()
        .route("/auth/dashboard-access/:app_id/", post(dashboard_access))
        .route("/auth/logout/", post(api_not_implemented))
}
