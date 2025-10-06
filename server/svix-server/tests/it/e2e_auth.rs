//! Module that test the dashboard-access endpoint and associated JWT tokens. This module will test
//! that the tokens returned by the endpoint have restricted functionality and that the response
//! from the endpoint is valid in the process.

use reqwest::StatusCode;
use serde::de::IgnoredAny;
use serde_json::Value;
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
