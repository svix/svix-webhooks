// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    client_secret_jwt_params_in::ClientSecretJwtParamsIn,
    oauth2_auth_method_in_out::Oauth2AuthMethodInOut,
    oauth2_grant_type_in_out::Oauth2GrantTypeInOut,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointOauthConfigIn {
    /// The OAuth grant type.
    #[serde(rename = "grantType")]
    pub grant_type: Oauth2GrantTypeInOut,

    #[serde(rename = "authMethod")]
    pub auth_method: Oauth2AuthMethodInOut,

    /// Optional JWT parameters. Only required for `clientSecretJwt`
    #[serde(rename = "jwtParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_params: Option<ClientSecretJwtParamsIn>,

    /// The client ID. Required for all authentication types.
    #[serde(rename = "clientId")]
    pub client_id: String,

    /// Optional client secret. This is only used for `clientSecretBasic` and
    /// `clientSecretPost`.
    ///
    /// For `clientSecretBasic`, the secret will be appended to the
    /// `Authorization` header. For `clientSecretPost`, this will be added
    /// to the body in a `client_secret` parameter.
    #[serde(rename = "clientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// The URL of the authorization server.
    #[serde(rename = "tokenUrl")]
    pub token_url: String,

    /// Optional OAuth scopes added to the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Extra parameters added to the request body as key-value pairs.
    #[serde(rename = "extraParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::BTreeMap<String, String>>,

    /// For `refreshToken` grant type.
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// Optional headers added to the token request.
    #[serde(rename = "extraHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_headers: Option<std::collections::BTreeMap<String, String>>,
}

impl EndpointOauthConfigIn {
    pub fn new(
        grant_type: Oauth2GrantTypeInOut,
        auth_method: Oauth2AuthMethodInOut,
        client_id: String,
        token_url: String,
    ) -> Self {
        Self {
            grant_type,
            auth_method,
            jwt_params: None,
            client_id,
            client_secret: None,
            token_url,
            scopes: None,
            extra_params: None,
            refresh_token: None,
            extra_headers: None,
        }
    }
}
