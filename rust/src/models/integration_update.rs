// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntegrationUpdate {
    pub name: String,
}

impl IntegrationUpdate {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
