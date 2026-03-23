// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenGetNamespaceIn {
    pub name: String,
}

impl AuthTokenGetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
