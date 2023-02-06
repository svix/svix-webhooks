use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use std::iter;

use super::{EndpointSecretOut, EndpointSecretRotateIn};
use crate::{
    cfg::DefaultSignatureType,
    core::{
        cryptography::Encryption,
        operational_webhooks::{EndpointEvent, OperationalWebhook},
        permissions,
        types::{EndpointSecretInternal, ExpiringSigningKey, ExpiringSigningKeys},
    },
    ctx,
    db::models::endpoint,
    error::{HttpError, Result},
    v1::utils::{ApplicationEndpointPath, EmptyResponse, ValidatedJson},
    AppState,
};

pub(super) fn generate_secret(
    encryption: &Encryption,
    sig_type: &DefaultSignatureType,
) -> Result<EndpointSecretInternal> {
    match sig_type {
        DefaultSignatureType::Hmac256 => EndpointSecretInternal::generate_symmetric(encryption),
        DefaultSignatureType::Ed25519 => EndpointSecretInternal::generate_asymmetric(encryption),
    }
}

pub(super) const GET_ENDPOINT_SECRET_DESCRIPTION: &str = r#"
Get the endpoint's signing secret.

This is used to verify the authenticity of the webhook.
For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
"#;

pub(super) async fn get_endpoint_secret(
    State(AppState { ref db, cfg, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<EndpointSecretOut>> {
    let endp = ctx!(
        endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(EndpointSecretOut {
        key: endp.key.into_endpoint_secret(&cfg.encryption)?,
    }))
}

pub(super) const ROTATE_ENDPOINT_SECRET_DESCRIPTION: &str = "Rotates the endpoint's signing secret.  The previous secret will be valid for the next 24 hours.";

pub(super) async fn rotate_endpoint_secret(
    State(AppState {
        ref db,
        cfg,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointSecretRotateIn>,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let mut endp = ctx!(
        endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
            .one(db)
            .await
    )?
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
    let endp = ctx!(endp.update(db).await)?;

    op_webhooks
        .send_operational_webhook(
            &app.org_id,
            OperationalWebhook::EndpointUpdated(EndpointEvent::new(app.uid.as_ref(), &endp)),
        )
        .await?;

    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}
