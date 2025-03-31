// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApiTokenIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl ApiTokenIn {
    pub fn new(name: String) -> Self {
        Self { name, scopes: None }
    }
}
