// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::event_out::EventOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventStreamOut {
    pub data: Vec<EventOut>,

    pub done: bool,

    pub iterator: String,
}

impl EventStreamOut {
    pub fn new(
        data: Vec<EventOut>,
        done: bool,
        iterator: String,
    ) -> Self {
        Self {
            data,
            done,
            iterator,
        }
    }
}
