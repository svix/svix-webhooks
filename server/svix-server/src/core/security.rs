// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::{Debug, Formatter};

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

    Ok(signing_config.generate(claims).unwrap())
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

    Ok(signing_config.generate(claims).unwrap())
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

    Ok(keys.generate(claims).unwrap())
}
#[derive(Deserialize)]
#[serde(untagged)]
pub enum JwtSigningConfig {
    /// Variants that specify both key and algorithm to use
    Advanced(Box<JWTAlgorithm>),
    /// The variant used when the algorithm is not specified, defaults to HS256
    Default {
        #[serde(deserialize_with = "deserialize_hs256")]
        jwt_secret: HS256Key,
    },
}

/// A wrapper for the available JWT signing algorithms exposed by `jwt-simple`
#[derive(Deserialize)]
#[serde(tag = "jwt_algorithm", content = "jwt_secret")]
pub enum JWTAlgorithm {
    #[serde(deserialize_with = "deserialize_hs256")]
    HS256(HS256Key),
    #[serde(deserialize_with = "deserialize_hs384")]
    HS384(HS384Key),
    #[serde(deserialize_with = "deserialize_hs512")]
    HS512(HS512Key),
    #[serde(deserialize_with = "deserialize_rs256")]
    RS256(RS256KeyPair),
    #[serde(deserialize_with = "deserialize_rs384")]
    RS384(RS384KeyPair),
    #[serde(deserialize_with = "deserialize_rs512")]
    RS512(RS512KeyPair),
    #[serde(deserialize_with = "deserialize_eddsa")]
    EdDSA(Ed25519KeyPair),
}

impl JwtSigningConfig {
    pub fn generate(
        &self,
        claims: JWTClaims<CustomClaim>,
    ) -> std::result::Result<String, jwt_simple::Error> {
        match self {
            JwtSigningConfig::Advanced(a) => match a.as_ref() {
                JWTAlgorithm::HS256(key) => key.authenticate(claims),
                JWTAlgorithm::HS384(key) => key.authenticate(claims),
                JWTAlgorithm::HS512(key) => key.authenticate(claims),
                JWTAlgorithm::RS256(key) => key.sign(claims),
                JWTAlgorithm::RS384(key) => key.sign(claims),
                JWTAlgorithm::RS512(key) => key.sign(claims),
                JWTAlgorithm::EdDSA(key) => key.sign(claims),
            },
            JwtSigningConfig::Default { jwt_secret } => jwt_secret.authenticate(claims),
        }
    }

    pub fn verify_token(
        &self,
        token: &str,
        options: Option<VerificationOptions>,
    ) -> std::result::Result<JWTClaims<CustomClaim>, jwt_simple::Error> {
        match self {
            JwtSigningConfig::Advanced(a) => match a.as_ref() {
                JWTAlgorithm::HS256(key) => key.verify_token(token, options),
                JWTAlgorithm::HS384(key) => key.verify_token(token, options),
                JWTAlgorithm::HS512(key) => key.verify_token(token, options),
                JWTAlgorithm::RS256(key) => key.public_key().verify_token(token, options),
                JWTAlgorithm::RS384(key) => key.public_key().verify_token(token, options),
                JWTAlgorithm::RS512(key) => key.public_key().verify_token(token, options),
                JWTAlgorithm::EdDSA(key) => key.public_key().verify_token(token, options),
            },
            JwtSigningConfig::Default { jwt_secret } => jwt_secret.verify_token(token, options),
        }
    }
}

impl Debug for JwtSigningConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JwtSigningConfig::Advanced(a) => {
                    match a.as_ref() {
                        JWTAlgorithm::HS256(_) => "HS256",
                        JWTAlgorithm::HS384(_) => "HS384",
                        JWTAlgorithm::HS512(_) => "HS512",
                        JWTAlgorithm::RS256(_) => "RS256",
                        JWTAlgorithm::RS384(_) => "RS384",
                        JWTAlgorithm::RS512(_) => "RS512",
                        JWTAlgorithm::EdDSA(_) => "EdDSA",
                    }
                }
                JwtSigningConfig::Default { .. } => {
                    "HS256"
                }
            }
        )
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

fn deserialize_hs512<'de, D>(deserializer: D) -> std::result::Result<HS512Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS512Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_rs256<'de, D>(deserializer: D) -> std::result::Result<RS256KeyPair, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RS256KeyPair::from_pem(&String::deserialize(deserializer)?).unwrap())
}

fn deserialize_rs384<'de, D>(deserializer: D) -> std::result::Result<RS384KeyPair, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RS384KeyPair::from_pem(&String::deserialize(deserializer)?).unwrap())
}

fn deserialize_rs512<'de, D>(deserializer: D) -> std::result::Result<RS512KeyPair, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RS512KeyPair::from_pem(&String::deserialize(deserializer)?).unwrap())
}

fn deserialize_eddsa<'de, D>(deserializer: D) -> std::result::Result<Ed25519KeyPair, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Ed25519KeyPair::from_pem(&String::deserialize(deserializer)?).unwrap())
}
