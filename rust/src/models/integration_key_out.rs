// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IntegrationKeyOut {
    pub key: String,
}

impl IntegrationKeyOut {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
