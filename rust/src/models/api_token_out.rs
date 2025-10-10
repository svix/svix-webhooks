// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApiTokenOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// The GlobalApplicationToken's ID.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    pub token: String,
}

impl ApiTokenOut {
    pub fn new(created_at: String, id: String, token: String) -> Self {
        Self {
            created_at,
            expires_at: None,
            id,
            name: None,
            scopes: None,
            token,
        }
    }
}
