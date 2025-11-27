// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::message_attempt_failed_data::MessageAttemptFailedData;

/// Sent when a message delivery has failed (all of the retry attempts have been
/// exhausted) as a "message.attempt.exhausted" type or after it's failed four
/// times as a "message.attempt.failing" event.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptRecoveredEventData {
    /// The Application's ID.
    #[serde(rename = "appId")]
    pub app_id: String,

    /// The Application's UID.
    #[serde(rename = "appUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_uid: Option<String>,

    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    #[serde(rename = "lastAttempt")]
    pub last_attempt: MessageAttemptFailedData,

    /// The Message's UID.
    #[serde(rename = "msgEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_event_id: Option<String>,

    /// The Message's ID.
    #[serde(rename = "msgId")]
    pub msg_id: String,
}

impl MessageAttemptRecoveredEventData {
    pub fn new(
        app_id: String,
        endpoint_id: String,
        last_attempt: MessageAttemptFailedData,
        msg_id: String,
    ) -> Self {
        Self {
            app_id,
            app_uid: None,
            endpoint_id,
            last_attempt,
            msg_event_id: None,
            msg_id,
        }
    }
}
