// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::ingest_message_attempt_recovered_event_data::IngestMessageAttemptRecoveredEventData;

/// Sent on a successful dispatch after an earlier failure op webhook has
/// already been sent.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestMessageAttemptRecoveredEvent {
    pub data: IngestMessageAttemptRecoveredEventData,

    pub r#type: String,
}

impl IngestMessageAttemptRecoveredEvent {
    pub fn new(
        data: IngestMessageAttemptRecoveredEventData,
        r#type: String,
    ) -> Self {
        Self {
            data,
            r#type,
        }
    }
}
