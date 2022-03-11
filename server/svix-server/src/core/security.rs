// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use jwt_simple::prelude::*;
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::{
    cfg::Configuration,
    db::models::application,
    error::{Error, HttpError, Result},
};

use super::types::{ApplicationIdOrUid, OrganizationId};

/// The default org_id we use (useful for generating JWTs when testing).
fn org_id() -> OrganizationId {
    OrganizationId("org_23rb8YdGqMT0qIzpgGwdXfHirMu".to_owned())
}

fn to_internal_server_error(x: impl Display) -> HttpError {
    tracing::error!("Error: {}", x);
    HttpError::internal_server_errer(None, None)
}

pub struct Permissions {
    // scopes: t.Set[KeyScopes]
    pub org_id: OrganizationId,
}

#[async_trait]
impl<B> FromRequest<B> for Permissions
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let Extension(ref cfg) = Extension::<Configuration>::from_request(req)
            .await
            .map_err(to_internal_server_error)?;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;

        let claims = cfg
            .jwt_secret
            .key
            .verify_token::<NoCustomClaims>(bearer.token(), None)
            .map_err(|_| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;

        if let Some(org_id) = claims.subject {
            let org_id = OrganizationId(org_id);
            org_id.validate().map_err(|_| {
                HttpError::bad_request(
                    Some("bad_token".to_string()),
                    Some("`sub' is not a valid organization id.".to_string()),
                )
            })?;
            Ok(Permissions { org_id })
        } else {
            Err(
                HttpError::unauthorized(None, Some("Invalid token (missing `sub`).".to_string()))
                    .into(),
            )
        }
    }
}

#[derive(Deserialize)]
struct ApplicationPathParams {
    app_id: ApplicationIdOrUid,
}

pub struct AuthenticatedApplication {
    pub permissions: Permissions,
    pub app: application::Model,
}

#[async_trait]
impl<B> FromRequest<B> for AuthenticatedApplication
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let permissions = Permissions::from_request(req).await?;
        let Path(ApplicationPathParams { app_id }) =
            Path::<ApplicationPathParams>::from_request(req)
                .await
                .map_err(to_internal_server_error)?;
        let Extension(ref db) = Extension::<DatabaseConnection>::from_request(req)
            .await
            .map_err(to_internal_server_error)?;
        let app = application::Entity::secure_find_by_id_or_uid(
            permissions.org_id.clone(),
            app_id.to_owned(),
        )
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
        Ok(AuthenticatedApplication { permissions, app })
    }
}

pub fn generate_token(keys: &Keys) -> Result<String> {
    let claims = Claims::create(Duration::from_hours(24 * 365 * 10))
        .with_issuer(env!("CARGO_PKG_NAME"))
        .with_subject(org_id().0);
    Ok(keys.key.authenticate(claims).unwrap())
}

#[derive(Clone, Debug)]
pub struct Keys {
    key: HS256Key,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            key: HS256Key::from_bytes(secret),
        }
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use super::*;
    use crate::core::types::BaseId;

    pub fn generate_token_random_org(keys: &Keys) -> Result<String> {
        let claims = Claims::create(Duration::from_hours(24 * 365 * 10))
            .with_issuer(env!("CARGO_PKG_NAME"))
            .with_subject(OrganizationId::new(None, None).0);
        Ok(keys.key.authenticate(claims).unwrap())
    }
}
