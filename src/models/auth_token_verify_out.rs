// this file is @generated
use serde::{Deserialize, Serialize};

use super::auth_token_out::AuthTokenOut;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthTokenVerifyOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<AuthTokenOut>,
}

impl AuthTokenVerifyOut {
    pub fn new() -> Self {
        Self { token: None }
    }

    pub fn with_token(mut self, value: impl Into<Option<AuthTokenOut>>) -> Self {
        self.token = value.into();
        self
    }
}
