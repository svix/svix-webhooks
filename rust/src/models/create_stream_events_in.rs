// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    event_in::EventIn,
    stream_in::StreamIn,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateStreamEventsIn {
    pub events: Vec<EventIn>,

    /// Optionally creates a new Stream alongside the events.
    ///
    /// If the stream id or uid that is used in the path already exists, this
    /// argument is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<StreamIn>,
}

impl CreateStreamEventsIn {
    pub fn new(events: Vec<EventIn>) -> Self {
        Self {
            events,
            stream: None,
        }
    }
}
