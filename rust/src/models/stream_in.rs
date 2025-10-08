// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// The stream's name.
    pub name: String,

    /// An optional unique identifier for the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl StreamIn {
    pub fn new(name: String) -> Self {
        Self {
            metadata: None,
            name,
            uid: None,
        }
    }
}
