// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::ingest_message_attempt_exhausted_event_data::IngestMessageAttemptExhaustedEventData;

/// Sent when a message delivery has failed (all of the retry attempts have been
/// exhausted).
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestMessageAttemptExhaustedEvent {
    pub data: IngestMessageAttemptExhaustedEventData,

    pub r#type: String,
}

impl IngestMessageAttemptExhaustedEvent {
    pub fn new(
        data: IngestMessageAttemptExhaustedEventData,
        r#type: String,
    ) -> Self {
        Self {
            data,
            r#type,
        }
    }
}
