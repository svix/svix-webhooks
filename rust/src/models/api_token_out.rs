// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApiTokenOut {
    pub token: String,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl ApiTokenOut {
    pub fn new(token: String, id: String, created_at: String) -> Self {
        Self {
            token,
            id,
            name: None,
            created_at,
            expires_at: None,
            scopes: None,
        }
    }
}
