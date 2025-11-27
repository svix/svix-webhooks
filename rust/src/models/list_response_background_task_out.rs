// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::background_task_out::BackgroundTaskOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseBackgroundTaskOut {
    pub data: Vec<BackgroundTaskOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseBackgroundTaskOut {
    pub fn new(
        data: Vec<BackgroundTaskOut>,
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
