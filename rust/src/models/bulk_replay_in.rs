// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, status_code_class::StatusCodeClass};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BulkReplayIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    pub since: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageStatus>,

    #[serde(rename = "statusCodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code_class: Option<StatusCodeClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl BulkReplayIn {
    pub fn new(since: String) -> Self {
        Self {
            channel: None,
            event_types: None,
            since,
            status: None,
            status_code_class: None,
            tag: None,
            until: None,
        }
    }
}
