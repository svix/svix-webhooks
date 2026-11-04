// this file is @generated
use serde::{Deserialize, Serialize};

/// A single message in a batch of bulk-created messages
///
/// Practically identical to `MessageIn`, but without the `application` field
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BatchMessageIn {
    /// Optional unique identifier for the message
    ///
    /// This should not contain the '.' character
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The event type's name
    #[serde(rename = "eventType")]
    pub event_type: String,

    /// JSON payload to send as the request body of the webhook.
    ///
    /// We also support sending non-JSON payloads. Please contact us for more
    /// information.
    ///
    /// The serialized byte-length of this value should be under 1MiB. Please
    /// contact us if you need to send payloads large than 1MiB.
    pub payload: serde_json::Value,

    /// List of free-form identifiers that endpoints can filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<std::collections::BTreeSet<String>>,

    /// List of free-form tags that can be filtered by when listing messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeSet<String>>,

    /// Extra parameters to pass to Transformations (for future use)
    #[serde(rename = "transformationsParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations_params: Option<serde_json::Value>,

    /// The date and time at which the message will be delivered.
    ///
    /// Note that this time is best-effort-only. Must be at least one minute
    /// and no more than 24 hours in the future.
    #[serde(rename = "deliverAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Optional number of days to retain the message payload. Defaults to 90.
    /// Note that this is mutually exclusive with `payloadRetentionHours`.
    #[serde(rename = "payloadRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_retention_period: Option<i64>,

    /// Optional number of hours to retain the message payload. Note that
    /// this is mutually exclusive with `payloadRetentionPeriod`.
    #[serde(rename = "payloadRetentionHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_retention_hours: Option<i64>,
}

impl BatchMessageIn {
    pub fn new(event_type: String, payload: serde_json::Value) -> Self {
        Self {
            event_id: None,
            event_type,
            payload,
            channels: None,
            tags: None,
            transformations_params: None,
            deliver_at: None,
            payload_retention_period: None,
            payload_retention_hours: None,
        }
    }
}
