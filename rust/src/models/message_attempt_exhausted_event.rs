// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::message_attempt_exhausted_event_data::MessageAttemptExhaustedEventData;

/// Sent when a message delivery has failed (all of the retry attempts have been
/// exhausted).
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptExhaustedEvent {
    pub data: MessageAttemptExhaustedEventData,

    pub r#type: String,
}

impl MessageAttemptExhaustedEvent {
    pub fn new(
        data: MessageAttemptExhaustedEventData,
        r#type: String,
    ) -> Self {
        Self {
            data,
            r#type,
        }
    }
}
