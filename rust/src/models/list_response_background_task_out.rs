use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListResponseBackgroundTaskOut {
    pub data: Vec<super::BackgroundTaskOut>,
    pub done: bool,
    pub iterator: Option<String>,
    #[serde(rename = "prevIterator", skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseBackgroundTaskOut {
    pub fn new(
        data: Vec<super::BackgroundTaskOut>,
        done: bool,
        iterator: Option<String>,
    ) -> ListResponseBackgroundTaskOut {
        ListResponseBackgroundTaskOut {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
