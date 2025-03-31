// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApiTokenCensoredOut {
    #[serde(rename = "censoredToken")]
    pub censored_token: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// The ApplicationToken's ID.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl ApiTokenCensoredOut {
    pub fn new(censored_token: String, created_at: String, id: String) -> Self {
        Self {
            censored_token,
            created_at,
            expires_at: None,
            id,
            name: None,
            scopes: None,
        }
    }
}
