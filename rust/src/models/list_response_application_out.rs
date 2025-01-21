// this file is @generated
use serde::{Deserialize, Serialize};

use super::application_out::ApplicationOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseApplicationOut {
    pub data: Vec<ApplicationOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseApplicationOut {
    pub fn new(data: Vec<ApplicationOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
