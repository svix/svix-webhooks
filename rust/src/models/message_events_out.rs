use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageEventsOut {
    pub data: Vec<super::MessageOut>,
    pub done: bool,
    pub iterator: String,
}

impl MessageEventsOut {
    pub fn new(
        data: Vec<super::MessageOut>,
        done: bool,
        iterator: String,
    ) -> MessageEventsOut {
        MessageEventsOut {
            data,
            done,
            iterator,
        }
    }
}
