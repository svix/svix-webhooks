// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HubspotConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl HubspotConfig {
    pub fn new() -> Self {
        Self { secret: None }
    }
}

impl Default for HubspotConfig {
    fn default() -> Self {
        Self::new()
    }
}
