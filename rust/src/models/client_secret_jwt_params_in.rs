// this file is @generated
use serde::{Deserialize, Serialize};

use super::oauth_jws_signing_algorithm::OauthJwsSigningAlgorithm;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClientSecretJwtParamsIn {
    /// The base64-encoded secret used for signing the JWT.
    #[serde(rename = "secretBase64")]
    pub secret_base64: String,

    /// The algorithm used to sign the JWT.
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: OauthJwsSigningAlgorithm,

    /// Optional secret identifier.
    ///
    /// If supplied, this will be populated in the JWT header in the `kid`
    /// field.
    #[serde(rename = "secretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,

    /// Optional number of seconds after which the JWT should expire.
    ///
    /// Defaults to 300 seconds.
    #[serde(rename = "tokenExpirySecs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_expiry_secs: Option<u64>,
}

impl ClientSecretJwtParamsIn {
    pub fn new(secret_base64: String, signing_algorithm: OauthJwsSigningAlgorithm) -> Self {
        Self {
            secret_base64,
            signing_algorithm,
            secret_id: None,
            token_expiry_secs: None,
        }
    }
}
