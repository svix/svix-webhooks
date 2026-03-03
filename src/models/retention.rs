// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Retention {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

impl Retention {
    pub fn new() -> Self {
        Self {
            millis: None,
            bytes: None,
        }
    }
}
