// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::ingest_message_attempt_failing_event_data::IngestMessageAttemptFailingEventData;

/// Sent after a message has been failing for a few times.
/// It's sent on the fourth failure. It complements
/// `ingest.message.attempt.exhausted` which is sent after the last failure.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestMessageAttemptFailingEvent {
    pub data: IngestMessageAttemptFailingEventData,

    pub r#type: String,
}

impl IngestMessageAttemptFailingEvent {
    pub fn new(
        data: IngestMessageAttemptFailingEventData,
        r#type: String,
    ) -> Self {
        Self {
            data,
            r#type,
        }
    }
}
