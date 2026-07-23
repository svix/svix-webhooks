// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamIn {
    /// The stream's name.
    pub name: String,

    /// An optional unique identifier for the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
}

impl StreamIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            uid: None,
            metadata: None,
        }
    }
}
