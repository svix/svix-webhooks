// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_attempt_recovered_event_data::MessageAttemptRecoveredEventData;

/// Sent on a successful dispatch after an earlier failure op webhook has
/// already been sent.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptRecoveredEvent {
    pub data: MessageAttemptRecoveredEventData,

    pub r#type: String,
}

impl MessageAttemptRecoveredEvent {
    pub fn new(data: MessageAttemptRecoveredEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
