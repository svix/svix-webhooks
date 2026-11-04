// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenericResponse {
    #[serde(rename = "contentType")]
    pub content_type: String,

    pub body: String,
}

impl GenericResponse {
    pub fn new(content_type: String, body: String) -> Self {
        Self { content_type, body }
    }
}
