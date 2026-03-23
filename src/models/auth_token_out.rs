// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenOut {
    pub id: String,

    pub name: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    pub metadata: std::collections::HashMap<String, String>,

    pub owner_id: String,

    pub scopes: Vec<String>,

    /// Whether this token is currently enabled.
    pub enabled: bool,
}

impl AuthTokenOut {
    pub fn new(
        id: String,
        name: String,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
        metadata: std::collections::HashMap<String, String>,
        owner_id: String,
        scopes: Vec<String>,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            name,
            created,
            updated,
            expiry: None,
            metadata,
            owner_id,
            scopes,
            enabled,
        }
    }

    pub fn with_expiry(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.expiry = value.into();
        self
    }
}
