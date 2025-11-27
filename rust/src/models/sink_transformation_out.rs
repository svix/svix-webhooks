// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SinkTransformationOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    pub enabled: bool,
}

impl SinkTransformationOut {
    pub fn new(enabled: bool) -> Self {
        Self {
            code: None,
            enabled,
        }
    }
}
