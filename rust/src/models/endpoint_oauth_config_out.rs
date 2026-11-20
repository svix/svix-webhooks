// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    oauth2_auth_method_in_out::Oauth2AuthMethodInOut,
    oauth2_grant_type_in_out::Oauth2GrantTypeInOut,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointOauthConfigOut {
    #[serde(rename = "grantType")]
    pub grant_type: Oauth2GrantTypeInOut,

    #[serde(rename = "authMethod")]
    pub auth_method: Oauth2AuthMethodInOut,

    #[serde(rename = "clientId")]
    pub client_id: String,

    #[serde(rename = "tokenUrl")]
    pub token_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "extraParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "extraHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_headers: Option<std::collections::BTreeMap<String, String>>,
}
