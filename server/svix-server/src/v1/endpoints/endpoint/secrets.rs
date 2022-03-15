use axum::{
    extract::{Extension, Path},
    Json,
};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::iter;

use super::{EndpointSecretOut, EndpointSecretRotateIn};
use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, EndpointIdOrUid, EndpointSecret, ExpiringSigningKey,
            ExpiringSigningKeys,
        },
    },
    db::models::endpoint,
    error::{HttpError, Result},
    v1::utils::{EmptyResponse, ValidatedJson},
};

pub(super) async fn get_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<EndpointSecretOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(EndpointSecretOut { key: endp.key }))
}

pub(super) async fn rotate_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointSecretRotateIn>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let mut endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let now = Utc::now();
    let last_key = ExpiringSigningKey {
        key: endp.key.clone(),
        expiration: now + Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
    };

    if let Some(ref old_keys) = endp.old_keys {
        if old_keys.0.len() + 1 > ExpiringSigningKeys::MAX_OLD_KEYS {
            return Err(HttpError::bad_request(
                Some("limit_reached".to_owned()),
                Some(format!(
                    "You can only rotate a key {} times within the last {}.",
                    ExpiringSigningKeys::MAX_OLD_KEYS,
                    ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS
                )),
            )
            .into());
        }
    }

    let old_keys = endp.old_keys.take();

    let endp = endpoint::ActiveModel {
        key: Set(if let Some(key) = data.key {
            key
        } else {
            EndpointSecret::generate()?
        }),

        old_keys: Set(Some(ExpiringSigningKeys(
            iter::once(last_key)
                .chain(
                    old_keys
                        .map(|x| x.0.into_iter())
                        .unwrap_or_else(|| vec![].into_iter()),
                )
                .collect(),
        ))),
        ..endp.into()
    };
    endp.update(db).await?;

    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

#[cfg(test)]
pub mod tests {
    use reqwest::StatusCode;

    use crate::{
        test_util::{start_svix_server, IgnoredResponse},
        v1::endpoints::{
            application::tests::create_test_app,
            endpoint::{tests::post_endpoint_default, EndpointSecretOut},
        },
    };

    // Simply tests that upon rotating an endpoint secret that it differs from the prior one
    #[tokio::test]
    #[cfg_attr(not(feature = "integration_testing"), ignore)]
    async fn test_endpoint_secret_get_and_rotation() {
        let (client, _jh) = start_svix_server();

        const APP_NAME: &str = "v1EndpointSecretRotationTestApp";
        const EP_URI: &str = "http://v1EndpointSecretRotationTestEp.test";

        let app_id = create_test_app(&client, APP_NAME).await.unwrap();

        let ep = post_endpoint_default(&client, &app_id, EP_URI)
            .await
            .unwrap();

        let former_secret: EndpointSecretOut = client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let _: IgnoredResponse = client
            .post(
                &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
                serde_json::json!({ "key": null }),
                StatusCode::NO_CONTENT,
            )
            .await
            .unwrap();

        assert_ne!(
            former_secret,
            client
                .get(
                    &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                    StatusCode::OK
                )
                .await
                .unwrap()
        );

        let _: IgnoredResponse = client
            .post(
                &format!("api/v1/app/{}/endpoint/{}/secret/rotate/", app_id, ep.id),
                &former_secret,
                StatusCode::NO_CONTENT,
            )
            .await
            .unwrap();

        assert_eq!(
            former_secret,
            client
                .get(
                    &format!("api/v1/app/{}/endpoint/{}/secret/", app_id, ep.id),
                    StatusCode::OK
                )
                .await
                .unwrap()
        );
    }
}
