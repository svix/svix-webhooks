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

use super::types::{ApplicationId, ApplicationIdOrUid, OrganizationId};

/// The default org_id we use (useful for generating JWTs when testing).
pub fn default_org_id() -> OrganizationId {
    OrganizationId("org_23rb8YdGqMT0qIzpgGwdXfHirMu".to_owned())
}

fn to_internal_server_error(x: impl Display) -> HttpError {
    tracing::error!("Error: {}", x);
    HttpError::internal_server_errer(None, None)
}

pub struct Permissions {
    pub type_: KeyType,
    pub org_id: OrganizationId,
    pub app_id: Option<ApplicationId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyType {
    Organization,
    Application,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomClaim {
    #[serde(rename = "org", default, skip_serializing_if = "Option::is_none")]
    organization: Option<String>,
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
            .verify_token::<CustomClaim>(bearer.token(), None)
            .map_err(|_| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;

        let bad_token = |field: &str, id_type: &str| {
            HttpError::bad_request(
                Some("bad token".to_string()),
                Some(format!("`{}` is not a valid {} id", field, id_type)),
            )
        };

        // If there is an `org` field then it is an Application authentication
        if let Some(org_id) = claims.custom.organization {
            let org_id = OrganizationId(org_id);
            org_id
                .validate()
                .map_err(|_| bad_token("org", "organization"))?;

            if let Some(app_id) = claims.subject {
                let app_id = ApplicationId(app_id);
                app_id
                    .validate()
                    .map_err(|_| bad_token("sub", "application"))?;

                Ok(Permissions {
                    org_id,
                    app_id: Some(app_id),
                    type_: KeyType::Application,
                })
            } else {
                Err(HttpError::unauthorized(
                    None,
                    Some("Invalid token (missing `sub`).".to_string()),
                )
                .into())
            }
        }
        // Otherwsie it's an Organization authentication
        else if let Some(org_id) = claims.subject {
            let org_id = OrganizationId(org_id);
            org_id.validate().map_err(|_| {
                HttpError::bad_request(
                    Some("bad_token".to_string()),
                    Some("`sub' is not a valid organization id.".to_string()),
                )
            })?;
            Ok(Permissions {
                org_id,
                app_id: None,
                type_: KeyType::Organization,
            })
        } else {
            Err(
                HttpError::unauthorized(None, Some("Invalid token (missing `sub`).".to_string()))
                    .into(),
            )
        }
    }
}

pub struct AuthenticatedOrganization {
    pub permissions: Permissions,
}

#[async_trait]
impl<B> FromRequest<B> for AuthenticatedOrganization
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let permissions = Permissions::from_request(req).await?;
        if permissions.type_ == KeyType::Application {
            return Err(HttpError::unauthorized(
                None,
                Some(
                    "You are only permitted to perform operations under the Application type"
                        .to_owned(),
                ),
            )
            .into());
        }

        Ok(AuthenticatedOrganization { permissions })
    }
}

#[derive(Deserialize)]
struct ApplicationPathParams {
    app_id: ApplicationIdOrUid,
}

pub struct OrganizationAuthenticatedApplication {
    pub permissions: Permissions,
    pub app: application::Model,
}

#[async_trait]
impl<B> FromRequest<B> for OrganizationAuthenticatedApplication
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let permissions = Permissions::from_request(req).await?;

        if permissions.type_ == KeyType::Application {
            return Err(HttpError::unauthorized(
                None,
                Some(
                    "You are only permitted to perform operations under the Application type"
                        .to_owned(),
                ),
            )
            .into());
        }

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
        Ok(OrganizationAuthenticatedApplication { permissions, app })
    }
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

pub fn generate_org_token(keys: &Keys, org_id: OrganizationId) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim { organization: None },
        Duration::from_hours(24 * 365 * 10),
    )
    .with_issuer(env!("CARGO_PKG_NAME"))
    .with_subject(org_id.0);
    Ok(keys.key.authenticate(claims).unwrap())
}

pub fn generate_app_token(
    keys: &Keys,
    org_id: OrganizationId,
    app_id: ApplicationId,
) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: Some(org_id.0),
        },
        Duration::from_hours(24 * 28),
    )
    .with_issuer(env!("CARGO_PKG_NAME"))
    .with_subject(app_id.0);
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
