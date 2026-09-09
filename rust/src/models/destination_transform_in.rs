// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DestinationTransformIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl DestinationTransformIn {
    pub fn new() -> Self {
        Self { code: None }
    }
}

impl Default for DestinationTransformIn {
    fn default() -> Self {
        Self::new()
    }
}
