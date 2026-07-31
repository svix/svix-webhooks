// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointTransformationIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl EndpointTransformationIn {
    pub fn new() -> Self {
        Self {
            code: None,
            enabled: None,
        }
    }
}

impl Default for EndpointTransformationIn {
    fn default() -> Self {
        Self::new()
    }
}
