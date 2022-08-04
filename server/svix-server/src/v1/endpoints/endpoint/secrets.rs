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
    cfg::{Configuration, DefaultSignatureType},
    core::{
        cryptography::Encryption,
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, EndpointIdOrUid, EndpointSecretInternal, ExpiringSigningKey,
            ExpiringSigningKeys,
        },
    },
    db::models::endpoint,
    error::{HttpError, Result},
    v1::utils::{EmptyResponse, ValidatedJson},
};

pub(super) fn generate_secret(
    encryption: &Encryption,
    sig_type: &DefaultSignatureType,
) -> Result<EndpointSecretInternal> {
    EndpointSecretInternal::generate(encryption, sig_type.into())
}

pub(super) async fn get_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(cfg): Extension<Configuration>,
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
    Ok(Json(EndpointSecretOut {
        key: endp.key.into_endpoint_secret(&cfg.encryption)?,
    }))
}

pub(super) async fn rotate_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(cfg): Extension<Configuration>,
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
            EndpointSecretInternal::from_endpoint_secret(key, &cfg.encryption)?
        } else {
            generate_secret(&cfg.encryption, &cfg.default_signature_type)?
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
