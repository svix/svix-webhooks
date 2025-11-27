// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::ingest_source_out::IngestSourceOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseIngestSourceOut {
    pub data: Vec<IngestSourceOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseIngestSourceOut {
    pub fn new(
        data: Vec<IngestSourceOut>,
        done: bool,
    ) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
