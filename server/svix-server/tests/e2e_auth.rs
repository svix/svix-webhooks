//! Module that test the dashboard-access endpoint and associated JWT tokens. This module will test
//! that the tokens returned by the endpoint have restricted functionality and that the response
//! from the endpoint is valid in the process.

use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashSet;

use svix_server::v1::endpoints::auth::{
    AppPortalAccessIn, DashboardAccessOut, OneTimeTokenIn, OneTimeTokenOut,
};
use svix_server::{core::types::ApplicationId, v1::endpoints::application::ApplicationOut};

mod utils;
use utils::{
    common_calls::{app_portal_access, application_in},
    start_svix_server, IgnoredResponse,
};

#[tokio::test]
/// Users with application-level tokens should only be allowed to read the information related to
/// their one application. All other endpoints should error.
async fn test_restricted_application_access() {
    let (client, _jh) = start_svix_server().await;

    let app_id: ApplicationId = client
        .post::<_, ApplicationOut>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;
    let app_id_2: ApplicationId = client
        .post::<_, ApplicationOut>(
            "api/v1/app/",
            application_in("TEST_APP_NAME_2"),
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;

    let client = app_portal_access(&client, &app_id, Default::default()).await;

    // CREATE, UPDATE, DELETE, and LIST ops
    let _: IgnoredResponse = client
        .post(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::FORBIDDEN,
        )
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .put(
            &format!("api/v1/app/{app_id}/"),
            application_in("TEST_APP_NAME"),
            StatusCode::FORBIDDEN,
        )
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{app_id}/"), StatusCode::FORBIDDEN)
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .get("api/v1/app/", StatusCode::FORBIDDEN)
        .await
        .unwrap();

    // READ should succeed when accessing the app_id the token is auhtorized for but no others
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{app_id_2}/"), StatusCode::NOT_FOUND)
        .await
        .unwrap();
    let _: ApplicationOut = client
        .get(&format!("api/v1/app/{app_id}/"), StatusCode::OK)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_dashboard_access_without_body() {
    let (client, _jh) = start_svix_server().await;

    let app_id: ApplicationId = client
        .post::<_, ApplicationOut>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;

    // We just need to ensure we get an OK response without a body.
    let _: IgnoredResponse = client
        .post(
            &format!("api/v1/auth/dashboard-access/{app_id}/"),
            (),
            StatusCode::OK,
        )
        .await
        .unwrap();
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct OneTimeTokenInner {
    app_id: String,
    one_time_token: String,
    region: String,
}

#[tokio::test]
async fn test_app_portal_access_one_time_tokens() {
    if let Ok("none") = std::env::var("SVIX_CACHE_TYPE").as_deref() {
        // Noop for configurations where cache is disabled.
        // The token exchange depends on cache!
        return;
    }

    let (client, _jh) = start_svix_server().await;

    let app_id: ApplicationId = client
        .post::<_, ApplicationOut>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::CREATED,
        )
        .await
        .unwrap()
        .id;

    let DashboardAccessOut { url, token } = client
        .post(
            &format!("api/v1/auth/app-portal-access/{app_id}/"),
            AppPortalAccessIn {
                feature_flags: HashSet::new(),
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    let url = url::Url::parse(&url).unwrap();
    let OneTimeTokenInner { one_time_token, .. } = serde_json::from_slice(
        &base64::decode(url.fragment().unwrap().strip_prefix("key=").unwrap()).unwrap(),
    )
    .unwrap();

    let OneTimeTokenOut {
        token: exchanged_token,
    } = client
        .post(
            "api/v1/auth/one-time-token/",
            OneTimeTokenIn {
                one_time_token: one_time_token.clone(),
            },
            StatusCode::OK,
        )
        .await
        .unwrap();

    assert_eq!(exchanged_token, token);

    // Assert they can only be used once
    let _: IgnoredResponse = client
        .post(
            "api/v1/auth/one-time-token/",
            OneTimeTokenIn { one_time_token },
            StatusCode::UNAUTHORIZED,
        )
        .await
        .unwrap();
}
