// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationTokenExpireIn {
    /// How many seconds until the old key is expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,

    /// An optional list of session ids.
    ///
    /// If any session ids are specified, only Application tokens created with
    /// that session id will be expired.
    #[serde(rename = "sessionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_ids: Option<Vec<String>>,
}

impl ApplicationTokenExpireIn {
    pub fn new() -> Self {
        Self {
            expiry: None,
            session_ids: None,
        }
    }
}
