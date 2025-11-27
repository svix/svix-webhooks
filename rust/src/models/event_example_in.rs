// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventExampleIn {
    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    /// If the event type schema contains an array of examples, chooses which
    /// one to send.
    ///
    /// Defaults to the first example. Ignored if the schema doesn't contain an
    /// array of examples.
    #[serde(rename = "exampleIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_index: Option<i32>,
}

impl EventExampleIn {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            example_index: None,
        }
    }
}
