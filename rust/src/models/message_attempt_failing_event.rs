// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::message_attempt_failing_event_data::MessageAttemptFailingEventData;

/// Sent after a message has been failing for a few times.
/// It's sent on the fourth failure. It complements `message.attempt.exhausted`
/// which is sent after the last failure.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptFailingEvent {
    pub data: MessageAttemptFailingEventData,

    pub r#type: String,
}

impl MessageAttemptFailingEvent {
    pub fn new(
        data: MessageAttemptFailingEventData,
        r#type: String,
    ) -> Self {
        Self {
            data,
            r#type,
        }
    }
}
