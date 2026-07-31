// this file is @generated
use serde::{Deserialize, Serialize};

use super::ingest_source_out::IngestSourceOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseIngestSourceOut {
    pub data: Vec<IngestSourceOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseIngestSourceOut {
    pub fn new(data: Vec<IngestSourceOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
