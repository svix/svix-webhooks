// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenOut {
    pub id: String,

    pub name: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    pub role: String,

    /// Whether this token is currently enabled.
    pub enabled: bool,
}

impl AdminAuthTokenOut {
    pub fn new(
        id: String,
        name: String,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
        role: String,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            name,
            created,
            updated,
            expiry: None,
            role,
            enabled,
        }
    }

    pub fn with_expiry(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.expiry = value.into();
        self
    }
}
