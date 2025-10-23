//! Module that test the dashboard-access endpoint and associated JWT tokens. This module will test
//! that the tokens returned by the endpoint have restricted functionality and that the response
//! from the endpoint is valid in the process.

use rand::distributions::DistString;
use reqwest::StatusCode;
use serde::de::IgnoredAny;
use serde_json::{json, Value};
use svix_server::{
    core::{
        security::{INVALID_TOKEN_ERR, JWT_SECRET_ERR},
        types::ApplicationId,
    },
    v1::endpoints::application::ApplicationOut,
};

use crate::utils::{
    common_calls::{app_portal_access, application_in},
    get_default_test_config, start_svix_server,
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
    let _: IgnoredAny = client
        .post(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::FORBIDDEN,
        )
        .await
        .unwrap();
    let _: IgnoredAny = client
        .put(
            &format!("api/v1/app/{app_id}/"),
            application_in("TEST_APP_NAME"),
            StatusCode::FORBIDDEN,
        )
        .await
        .unwrap();
    client
        .delete(&format!("api/v1/app/{app_id}/"), StatusCode::FORBIDDEN)
        .await
        .unwrap();
    let _: IgnoredAny = client
        .get("api/v1/app/", StatusCode::FORBIDDEN)
        .await
        .unwrap();

    // READ should succeed when accessing the app_id the token is authorized for but no others
    let _: IgnoredAny = client
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
    let _: IgnoredAny = client
        .post(
            &format!("api/v1/auth/dashboard-access/{app_id}/"),
            (),
            StatusCode::OK,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_invalid_auth_error_detail() {
    let (mut client, _jh) = start_svix_server().await;
    let cfg = get_default_test_config();
    let jwt_secret = match cfg.jwt_signing_config.as_ref() {
        svix_server::core::security::JwtSigningConfig::Default { jwt_secret } => {
            std::str::from_utf8(&jwt_secret.to_bytes())
                .unwrap()
                .to_owned()
        }

        _ => return,
    };

    client.set_auth_header("some-nonsense-key".to_string());
    match client
        .post::<_, Value>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::UNAUTHORIZED,
        )
        .await
    {
        Ok(Value::Object(i)) => {
            assert_eq!(i.get("detail").unwrap(), INVALID_TOKEN_ERR);
        }
        _ => {
            panic!("Unexpected response");
        }
    }
    client.set_auth_header(jwt_secret);
    match client
        .post::<_, Value>(
            "api/v1/app/",
            application_in("TEST_APP_NAME"),
            StatusCode::UNAUTHORIZED,
        )
        .await
    {
        Ok(Value::Object(i)) => {
            assert_eq!(i.get("detail").unwrap(), JWT_SECRET_ERR);
        }
        _ => {
            panic!("Unexpected response");
        }
    }
}

#[tokio::test]
async fn test_app_portal_access_with_application() {
    let (client, _jh) = start_svix_server().await;

    let app_uid = format!(
        "app-created-in-portal-{}",
        rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 15)
    );

    // app-portal-access without the application field fails
    let _: IgnoredAny = client
        .post(
            &format!("api/v1/auth/app-portal-access/{app_uid}/"),
            json!({
                "featureFlags": []
            }),
            StatusCode::NOT_FOUND,
        )
        .await
        .unwrap();

    // app-portal-access with application
    let _: IgnoredAny = client
        .post(
            &format!("api/v1/auth/app-portal-access/{app_uid}/"),
            json!({
                "featureFlags": [],
                "application": {
                    "name": "Test App Created With Portal Access",
                    "uid": app_uid,
                }
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // app was created
    let app: serde_json::Value = client
        .get(&format!("api/v1/app/{app_uid}/"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(app["uid"], app_uid);
    assert_eq!(app["name"], "Test App Created With Portal Access");

    // Access portal again with application field - should be ignored since app exists
    let _: IgnoredAny = client
        .post(
            &format!("api/v1/auth/app-portal-access/{app_uid}/"),
            json!({
                "featureFlags": [],
                "application": {
                    "name": "Updated name will be ignored",
                    "uid": app_uid,
                }
            }),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // Verify the app name didn't change
    let app_after: serde_json::Value = client
        .get(&format!("api/v1/app/{app_uid}/"), StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(app_after["name"], "Test App Created With Portal Access");

    // UID in path must match UID in body
    let _: IgnoredAny = client
        .post(
            "api/v1/auth/app-portal-access/different-uid/",
            json!({
                "featureFlags": [],
                "application": {
                    "name": "Test App",
                    "uid": app_uid,  // This doesn't match the path
                }
            }),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();

    // UID must be set in body when creating
    let _: IgnoredAny = client
        .post(
            "api/v1/auth/app-portal-access/new-app-uid/",
            json!({
                "featureFlags": [],
                "application": {
                    "name": "Test App Without UID",
                    // Missing uid field
                }
            }),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}
