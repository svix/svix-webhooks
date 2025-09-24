// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_attempt_failed_data::MessageAttemptFailedData;

/// Sent when a message delivery has failed (all of the retry attempts have been
/// exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed
/// four times as a "ingest.message.attempt.failing" event, or after it's
/// recovered as a "ingest.message.attempt.recovered" event.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestMessageAttemptExhaustedEventData {
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

    /// The Source's ID.
    #[serde(rename = "sourceId")]
    pub source_id: String,
}

impl IngestMessageAttemptExhaustedEventData {
    pub fn new(
        endpoint_id: String,
        last_attempt: MessageAttemptFailedData,
        msg_id: String,
        source_id: String,
    ) -> Self {
        Self {
            endpoint_id,
            last_attempt,
            msg_event_id: None,
            msg_id,
            source_id,
        }
    }
}
