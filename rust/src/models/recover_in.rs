// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecoverIn {
    pub since: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl RecoverIn {
    pub fn new(since: String) -> Self {
        Self {
            since,
            until: None,
        }
    }
}
