// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_attempt_log::MessageAttemptLog;

/// Sent after message attempts are made. Contains metadata about message
/// attempts and their results. In order to reduce the frequency of webhooks,
/// these are sent in batches periodically.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptLogEvent {
    pub data: Vec<MessageAttemptLog>,

    pub r#type: String,
}

impl MessageAttemptLogEvent {
    pub fn new(data: Vec<MessageAttemptLog>, r#type: String) -> Self {
        Self { data, r#type }
    }
}
