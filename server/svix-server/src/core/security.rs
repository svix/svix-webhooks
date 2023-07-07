// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use http::request::Parts;
use jwt_simple::prelude::*;
use serde::Deserializer;

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

/// The default Operational Webhooks org_id
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

    let claims = parse_bearer(&state.cfg.jwt_signing_config, &bearer)
        .ok_or_else(|| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;
    let perms = permissions_from_jwt(claims)?;

    tracing::Span::current().record("org_id", perms.org_id().to_string());
    if let Some(app_id) = perms.app_id() {
        tracing::Span::current().record("app_id", app_id.to_string());
    }

    Ok(perms)
}

pub fn parse_bearer(
    signing_config: &JwtSigningConfig,
    bearer: &Bearer,
) -> Option<JWTClaims<CustomClaim>> {
    signing_config.verify_token(bearer.token(), None).ok()
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

pub fn generate_org_token(
    signing_config: &JwtSigningConfig,
    org_id: OrganizationId,
) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: None,
            feature_flags: Default::default(),
        },
        Duration::from_hours(24 * 365 * 10),
    )
    .with_issuer(JWT_ISSUER)
    .with_subject(org_id.0);

    Ok(signing_config.generate(claims))
}

pub fn generate_management_token(signing_config: &JwtSigningConfig) -> Result<String> {
    let claims = Claims::with_custom_claims(
        CustomClaim {
            organization: None,
            feature_flags: Default::default(),
        },
        Duration::from_mins(10),
    )
    .with_issuer(JWT_ISSUER)
    .with_subject(management_org_id());

    Ok(signing_config.generate(claims))
}

pub fn generate_app_token(
    keys: &JwtSigningConfig,
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

    Ok(keys.generate(claims))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum JwtSigningConfig {
    Advanced(Advanced),
    Default {
        #[serde(deserialize_with = "deserialize_hs256")]
        jwt_secret: HS256Key,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "jwt_algorithm", content = "jwt_secret")]
pub enum Advanced {
    #[serde(deserialize_with = "deserialize_hs256")]
    HS256(HS256Key),
    #[serde(deserialize_with = "deserialize_hs384")]
    HS384(HS384Key),
    #[serde(deserialize_with = "deserialize_rs256")]
    RS256(RS256KeyPair),
}

impl JwtSigningConfig {
    pub fn generate(&self, claims: JWTClaims<CustomClaim>) -> String {
        match self {
            JwtSigningConfig::Advanced(a) => match a {
                Advanced::HS256(key) => key.authenticate(claims).unwrap(),
                Advanced::HS384(key) => key.authenticate(claims).unwrap(),
                Advanced::RS256(key) => key.sign(claims).unwrap(),
            },
            JwtSigningConfig::Default { jwt_secret } => jwt_secret.authenticate(claims).unwrap(),
        }
    }

    pub fn verify_token(
        &self,
        token: &str,
        options: Option<VerificationOptions>,
    ) -> std::result::Result<JWTClaims<CustomClaim>, jwt_simple::Error> {
        match self {
            JwtSigningConfig::Advanced(a) => match a {
                Advanced::HS256(key) => key.verify_token(token, options),
                Advanced::HS384(key) => key.verify_token(token, options),
                Advanced::RS256(key) => key.public_key().verify_token(token, options),
            },
            JwtSigningConfig::Default { jwt_secret } => jwt_secret.verify_token(token, options),
        }
    }
}

fn deserialize_hs256<'de, D>(deserializer: D) -> std::result::Result<HS256Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS256Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_hs384<'de, D>(deserializer: D) -> std::result::Result<HS384Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS384Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_rs256<'de, D>(deserializer: D) -> std::result::Result<RS256KeyPair, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RS256KeyPair::from_pem(&String::deserialize(deserializer)?).unwrap())
}
