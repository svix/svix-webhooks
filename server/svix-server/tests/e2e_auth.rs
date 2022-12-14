//! Module that test the dashboard-access endpoint and associated JWT tokens. This module will test
//! that the tokens returned by the endpoint have restricted functionality and that the response
//! from the endpoint is valid in the process.

use reqwest::StatusCode;

use svix_server::{
    core::types::ApplicationId,
    v1::endpoints::{application::ApplicationOut, auth::DashboardAccessOut},
};

mod utils;
use utils::{common_calls::application_in, start_svix_server, IgnoredResponse, TestClient};

/// Accesses the dashboard-access endpoint and returns a new [`TestClient`] with an auth header set
/// to the returned token.
async fn dashboard_access(org_client: &TestClient, application_id: &ApplicationId) -> TestClient {
    let resp: DashboardAccessOut = org_client
        .post(
            &format!("api/v1/auth/dashboard-access/{application_id}/"),
            (),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let mut app_client = org_client.clone();
    app_client.set_auth_header(resp.token);

    app_client
}

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

    let client = dashboard_access(&client, &app_id).await;

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
