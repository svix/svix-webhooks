// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use http::request::Parts;
use jwt_simple::prelude::*;

use validator::Validate;

use crate::{
    ctx,
    error::{HttpError, Result},
    AppState,
};

use super::types::{ApplicationId, FeatureFlagSet, OrganizationId};

/// The default org_id we use (useful for generating JWTs when testing).
pub fn default_org_id() -> OrganizationId {
    OrganizationId("org_23rb8YdGqMT0qIzpgGwdXfHirMu".to_owned())
}

/// The default org_id we use (useful for generating JWTs when testing).
pub fn management_org_id() -> OrganizationId {
    OrganizationId("org_00000000000SvixManagement00".to_owned())
}

pub enum AccessLevel {
    Organization(OrganizationId),
    Application(OrganizationId, ApplicationId),
}

pub struct Permissions {
    pub access_level: AccessLevel,
    pub feature_flags: FeatureFlagSet,
}

impl Permissions {
    pub fn org_id(&self) -> OrganizationId {
        match &self.access_level {
            AccessLevel::Organization(org_id) => org_id.clone(),
            AccessLevel::Application(org_id, _) => org_id.clone(),
        }
    }

    pub fn app_id(&self) -> Option<ApplicationId> {
        match &self.access_level {
            AccessLevel::Organization(_) => None,
            AccessLevel::Application(_, app_id) => Some(app_id.clone()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomClaim {
    #[serde(rename = "org", default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(
        rename = "feature_flags",
        default,
        skip_serializing_if = "FeatureFlagSet::is_empty"
    )]
    pub feature_flags: FeatureFlagSet,
}

pub async fn permissions_from_bearer(parts: &mut Parts, state: &AppState) -> Result<Permissions> {
    let TypedHeader(Authorization(bearer)) =
        ctx!(TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await)?;

    let claims = parse_bearer(&state.cfg.jwt_secret, &bearer)
        .ok_or_else(|| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;
    let perms = permissions_from_jwt(claims)?;

    tracing::Span::current().record("org_id", perms.org_id().to_string());
    if let Some(app_id) = perms.app_id() {
        tracing::Span::current().record("app_id", app_id.to_string());
    }

    Ok(perms)
}

pub fn parse_bearer(key: &Keys, bearer: &Bearer) -> Option<JWTClaims<CustomClaim>> {
    key.key
        .verify_token::<CustomClaim>(bearer.token(), None)
        .ok()
}

pub fn permissions_from_jwt(claims: JWTClaims<CustomClaim>) -> Result<Permissions> {
    let bad_token = |field: &str, id_type: &str| {
        HttpError::bad_request(
            Some("bad token".to_string()),
            Some(format!("`{field}` is not a valid {id_type} id")),
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
                access_level: AccessLevel::Application(org_id, app_id),
                feature_flags: claims.custom.feature_flags,
            })
        } else {
            Err(
                HttpError::unauthorized(None, Some("Invalid token (missing `sub`).".to_string()))
                    .into(),
            )
        }
    }
    // Otherwise it's an Organization authentication
    else if let Some(org_id) = claims.subject {
        let org_id = OrganizationId(org_id);
        org_id.validate().map_err(|_| {
            HttpError::bad_request(
                Some("bad_token".to_string()),
                Some("`sub' is not a valid organization id.".to_string()),
            )
        })?;
        Ok(Permissions {
            access_level: AccessLevel::Organization(org_id),
            feature_flags: claims.custom.feature_flags,
        })
    } else {
        Err(
            HttpError::unauthorized(None, Some("Invalid token (missing `sub`).".to_string()))
                .into(),
        )
    }
}

const JWT_ISSUER: &str = env!("CARGO_PKG_NAME");

pub fn generate_org_token(keys: &Keys, org_id: OrganizationId) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: None,
            feature_flags: Default::default(),
        },
        Duration::from_hours(24 * 365 * 10),
    )
    .with_issuer(JWT_ISSUER)
    .with_subject(org_id.0);
    Ok(keys.key.authenticate(claims).unwrap())
}

pub fn generate_management_token(keys: &Keys) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: None,
            feature_flags: Default::default(),
        },
        Duration::from_mins(10),
    )
    .with_issuer(JWT_ISSUER)
    .with_subject(management_org_id());
    Ok(keys.key.authenticate(claims).unwrap())
}

pub fn generate_app_token(
    keys: &Keys,
    org_id: OrganizationId,
    app_id: ApplicationId,
    feature_flags: FeatureFlagSet,
) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: Some(org_id.0),
            feature_flags,
        },
        Duration::from_hours(24 * 28),
    )
    .with_issuer(JWT_ISSUER)
    .with_subject(app_id.0);
    Ok(keys.key.authenticate(claims).unwrap())
}

#[derive(Clone, Debug)]
pub struct Keys {
    pub key: HS256Key,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            key: HS256Key::from_bytes(secret),
        }
    }
}
