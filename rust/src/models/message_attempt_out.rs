// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    message_attempt_trigger_type::MessageAttemptTriggerType, message_out::MessageOut,
    message_status::MessageStatus, message_status_text::MessageStatusText,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptOut {
    pub url: String,

    pub response: String,

    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i16,

    /// Response duration in milliseconds.
    #[serde(rename = "responseDurationMs")]
    pub response_duration_ms: i64,

    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    #[serde(rename = "triggerType")]
    pub trigger_type: MessageAttemptTriggerType,

    /// The Message's ID.
    #[serde(rename = "msgId")]
    pub msg_id: String,

    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// The MessageAttempt's ID.
    pub id: String,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<MessageOut>,
}

impl MessageAttemptOut {
    pub fn new(
        url: String,
        response: String,
        response_status_code: i16,
        response_duration_ms: i64,
        status: MessageStatus,
        status_text: MessageStatusText,
        trigger_type: MessageAttemptTriggerType,
        msg_id: String,
        endpoint_id: String,
        id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            url,
            response,
            response_status_code,
            response_duration_ms,
            status,
            status_text,
            trigger_type,
            msg_id,
            endpoint_id,
            id,
            timestamp,
            msg: None,
        }
    }
}
