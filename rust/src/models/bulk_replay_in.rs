// this file is @generated
use serde::{Deserialize, Serialize};

use super::{message_status::MessageStatus, status_code_class::StatusCodeClass};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BulkReplayIn {
    pub since: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<std::collections::BTreeSet<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageStatus>,

    #[serde(rename = "statusCodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code_class: Option<StatusCodeClass>,
}

impl BulkReplayIn {
    pub fn new(since: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            since,
            until: None,
            event_types: None,
            channel: None,
            tag: None,
            status: None,
            status_code_class: None,
        }
    }
}
