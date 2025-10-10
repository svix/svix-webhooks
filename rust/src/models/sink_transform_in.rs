// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SinkTransformIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl SinkTransformIn {
    pub fn new() -> Self {
        Self { code: None }
    }
}
