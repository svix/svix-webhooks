// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
    pub example_index: Option<u64>,

    /// Custom payload to send as an example
    ///
    /// This is only available to allow-listed customers and should otherwise
    /// not be passed. Please contact us if you need access to this
    /// functionality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl EventExampleIn {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            example_index: None,
            payload: None,
        }
    }
}
