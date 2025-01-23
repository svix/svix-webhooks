// this file is @generated
use serde::{Deserialize, Serialize};

use super::integration_out::IntegrationOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseIntegrationOut {
    pub data: Vec<IntegrationOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseIntegrationOut {
    pub fn new(data: Vec<IntegrationOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
