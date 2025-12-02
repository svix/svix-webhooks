// this file is @generated
use serde::{Deserialize, Serialize};

use super::{http_attempt_times::HttpAttemptTimes, message_status::MessageStatus};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAttemptLog {
    /// The Application's ID.
    #[serde(rename = "appId")]
    pub app_id: String,

    /// The Application's UID.
    #[serde(rename = "appUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_uid: Option<String>,

    #[serde(rename = "attemptCount")]
    pub attempt_count: u16,

    #[serde(rename = "attemptEnd")]
    pub attempt_end: String,

    /// The MessageAttempt's ID.
    #[serde(rename = "attemptId")]
    pub attempt_id: String,

    #[serde(rename = "attemptStart")]
    pub attempt_start: String,

    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// The event type's name
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    #[serde(rename = "httpTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_times: Option<HttpAttemptTimes>,

    #[serde(rename = "msgCreated")]
    pub msg_created: String,

    /// The Message's UID.
    #[serde(rename = "msgEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_event_id: Option<String>,

    /// The Message's ID.
    #[serde(rename = "msgId")]
    pub msg_id: String,

    /// The Environment's ID.
    #[serde(rename = "orgId")]
    pub org_id: String,

    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i16,

    pub status: MessageStatus,
}

impl MessageAttemptLog {
    pub fn new(
        app_id: String,
        attempt_count: u16,
        attempt_end: String,
        attempt_id: String,
        attempt_start: String,
        endpoint_id: String,
        msg_created: String,
        msg_id: String,
        org_id: String,
        response_status_code: i16,
        status: MessageStatus,
    ) -> Self {
        Self {
            app_id,
            app_uid: None,
            attempt_count,
            attempt_end,
            attempt_id,
            attempt_start,
            endpoint_id,
            event_type: None,
            http_times: None,
            msg_created,
            msg_event_id: None,
            msg_id,
            org_id,
            response_status_code,
            status,
        }
    }
}
