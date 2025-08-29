// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    message_attempt_trigger_type::MessageAttemptTriggerType, message_out::MessageOut,
    message_status::MessageStatus, message_status_text::MessageStatusText,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptOut {
    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// The MessageAttempt's ID.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<MessageOut>,

    /// The Message's ID.
    #[serde(rename = "msgId")]
    pub msg_id: String,

    pub response: String,

    /// Response duration in milliseconds.
    #[serde(rename = "responseDurationMs")]
    pub response_duration_ms: i32,

    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i16,

    pub status: MessageStatus,

    #[serde(rename = "statusText")]
    pub status_text: MessageStatusText,

    pub timestamp: String,

    #[serde(rename = "triggerType")]
    pub trigger_type: MessageAttemptTriggerType,

    pub url: String,
}

impl MessageAttemptOut {
    pub fn new(
        endpoint_id: String,
        id: String,
        msg_id: String,
        response: String,
        response_duration_ms: i32,
        response_status_code: i16,
        status: MessageStatus,
        status_text: MessageStatusText,
        timestamp: String,
        trigger_type: MessageAttemptTriggerType,
        url: String,
    ) -> Self {
        Self {
            endpoint_id,
            id,
            msg: None,
            msg_id,
            response,
            response_duration_ms,
            response_status_code,
            status,
            status_text,
            timestamp,
            trigger_type,
            url,
        }
    }
}
