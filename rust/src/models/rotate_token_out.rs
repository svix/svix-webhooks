// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RotateTokenOut {
    #[serde(rename = "ingestUrl")]
    pub ingest_url: String,
}

impl RotateTokenOut {
    pub fn new(ingest_url: String) -> Self {
        Self {
            ingest_url,
        }
    }
}
