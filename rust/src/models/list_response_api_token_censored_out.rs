// this file is @generated
use serde::{Deserialize, Serialize};

use super::api_token_censored_out::ApiTokenCensoredOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseApiTokenCensoredOut {
    pub data: Vec<ApiTokenCensoredOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseApiTokenCensoredOut {
    pub fn new(data: Vec<ApiTokenCensoredOut>, done: bool) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
