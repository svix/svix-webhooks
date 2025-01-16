// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntegrationIn {
    pub name: String,
}

impl IntegrationIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
