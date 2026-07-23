// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointTransformationOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl EndpointTransformationOut {
    pub fn new() -> Self {
        Self {
            code: None,
            enabled: None,
            variables: None,
            updated_at: None,
        }
    }
}

impl Default for EndpointTransformationOut {
    fn default() -> Self {
        Self::new()
    }
}
