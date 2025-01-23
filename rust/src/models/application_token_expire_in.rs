// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationTokenExpireIn {
    /// How many seconds until the old key is expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,
}

impl ApplicationTokenExpireIn {
    pub fn new() -> Self {
        Self { expiry: None }
    }
}
