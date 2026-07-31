use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use svix_server_derive::aide_annotate;

use super::{EndpointSecretOut, EndpointSecretRotateIn};
use crate::{
    AppState,
    cfg::{ConfigurationInner, DefaultSignatureType},
    core::{
        can_fail::CanFail,
        cryptography::Encryption,
        message_app::CreateMessageApp,
        operational_webhooks::{EndpointEvent, OperationalWebhook},
        permissions,
        types::{EndpointSecret, EndpointSecretInternal, ExpiringSigningKey, ExpiringSigningKeys},
    },
    db::models::endpoint,
    error::{HttpError, Result},
    v1::utils::{ApplicationEndpointPath, NoContent, ValidatedJson},
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

pub struct SigningKeysData {
    pub current_key: Option<EndpointSecretInternal>,
    pub old_keys: Option<ExpiringSigningKeys>,
}

/// Takes in a `SigningKeysData` corresponding to the latest 'current_key / old_keys' for the endpoint/sink,
/// and produces a new `SigningKeysData` by performing a key rotation.
///
/// The new key is taken from the input `EndpointSecretRotateIn`, or generated if not provided.
pub fn rotate_key(
    current_data: SigningKeysData,
    new_key: Option<EndpointSecret>,
    cfg: &ConfigurationInner,
    grace_period: Duration,
) -> Result<SigningKeysData> {
    let now = Utc::now();
    let last_key = current_data
        .current_key
        .clone()
        .map(|current_key| ExpiringSigningKey {
            key: current_key,
            expiration: now + grace_period,
        });

    let mut unexpired_old_keys = current_data
        .old_keys
        .as_ref()
        .map(|k| k.unexpired_keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    if let Some(last_key) = last_key.filter(|k| k.is_unexpired()) {
        unexpired_old_keys.push(last_key)
    }

    unexpired_old_keys.sort_by_key(|x| x.expiration);

    if unexpired_old_keys.len() > ExpiringSigningKeys::MAX_OLD_KEYS {
        return Err(HttpError::bad_request(
            Some("limit_reached".to_owned()),
            Some(format!(
                "You can only rotate a key {} times; please wait for old keys to expire. Your next key expires at {}.",
                ExpiringSigningKeys::MAX_OLD_KEYS,
                unexpired_old_keys.first().expect("we just checked that this is non-empty").expiration
            )),
        )
        .into());
    }

    Ok(SigningKeysData {
        current_key: Some(if let Some(key) = new_key {
            EndpointSecretInternal::from_endpoint_secret(key, &cfg.encryption)?
        } else {
            generate_secret(&cfg.encryption, &cfg.default_signature_type)?
        }),
        old_keys: Some(ExpiringSigningKeys(unexpired_old_keys)),
    })
}

/// Get the endpoint's signing secret.
///
/// This is used to verify the authenticity of the webhook.
/// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
#[aide_annotate(op_id = "v1.endpoint.get-secret")]
pub(super) async fn get_endpoint_secret(
    State(AppState { ref db, cfg, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<EndpointSecretOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(EndpointSecretOut {
        key: endp.key.into_endpoint_secret(&cfg.encryption)?,
    }))
}

/// Rotates the endpoint's signing secret.  The previous secret will be valid for the next 24 hours.
#[aide_annotate(op_id = "v1.endpoint.rotate-secret")]
pub(super) async fn rotate_endpoint_secret(
    State(AppState {
        ref db,
        cfg,
        ref op_webhooks,
        ref cache,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointSecretRotateIn>,
) -> Result<NoContent> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let grace_period = data.grace_period();

    let new_data = rotate_key(
        SigningKeysData {
            current_key: Some(endp.key.clone()),
            old_keys: endp.old_keys.clone(),
        },
        data.key,
        &cfg,
        grace_period,
    )?;

    let endp = endpoint::ActiveModel {
        key: Set(new_data.current_key.unwrap()),
        old_keys: Set(new_data.old_keys),
        ..endp.into()
    };
    let endp = endp.update(db).await?;

    CreateMessageApp::invalidate(cache, &app.id, &app.org_id)
        .await
        .can_fail("invalidating CMA cache");

    op_webhooks
        .send_operational_webhook(
            &app.org_id,
            OperationalWebhook::EndpointUpdated(EndpointEvent::new(app.uid.as_ref(), &endp)),
        )
        .await?;

    Ok(NoContent)
}
