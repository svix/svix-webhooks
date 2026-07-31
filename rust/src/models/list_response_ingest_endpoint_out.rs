// this file is @generated
use serde::{Deserialize, Serialize};

use super::ingest_endpoint_out::IngestEndpointOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseIngestEndpointOut {
    pub data: Vec<IngestEndpointOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseIngestEndpointOut {
    pub fn new(data: Vec<IngestEndpointOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
