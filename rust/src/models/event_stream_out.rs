// this file is @generated
use serde::{Deserialize, Serialize};

use super::event_out::EventOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventStreamOut {
    pub data: Vec<EventOut>,

    pub iterator: String,

    pub done: bool,
}

impl EventStreamOut {
    pub fn new(data: Vec<EventOut>, iterator: String, done: bool) -> Self {
        Self {
            data,
            iterator,
            done,
        }
    }
}
