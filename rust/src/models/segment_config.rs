// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SegmentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl SegmentConfig {
    pub fn new() -> Self {
        Self { secret: None }
    }
}

impl Default for SegmentConfig {
    fn default() -> Self {
        Self::new()
    }
}
