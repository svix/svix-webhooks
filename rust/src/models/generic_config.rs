// this file is @generated
use serde::{Deserialize, Serialize};

use super::generic_response::GenericResponse;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenericConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<GenericResponse>,
}

impl GenericConfig {
    pub fn new() -> Self {
        Self { response: None }
    }
}

impl Default for GenericConfig {
    fn default() -> Self {
        Self::new()
    }
}
