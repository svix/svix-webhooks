// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplayIn {
    pub since: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl ReplayIn {
    pub fn new(since: String) -> Self {
        Self {
            since,
            until: None,
        }
    }
}
