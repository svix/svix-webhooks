// this file is @generated
use serde::{Deserialize, Serialize};

use super::application_in::ApplicationIn;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageIn {
    /// Optionally creates a new application alongside the message.
    ///
    /// If the application id or uid that is used in the path already exists,
    /// this argument is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationIn>,

    /// List of free-form identifiers that endpoints can filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    /// The date and time at which the message will be delivered.
    ///
    /// Note that this time is best-effort-only. Must be at least one minute and
    /// no more than 24 hours in the future.
    ///
    /// RFC3339 date string.
    #[serde(rename = "deliverAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_at: Option<String>,

    /// Optional unique identifier for the message
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
    pub payload: serde_json::Value,

    /// Optional number of hours to retain the message payload. Note that this
    /// is mutually exclusive with `payloadRetentionPeriod`.
    #[serde(rename = "payloadRetentionHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_retention_hours: Option<i32>,

    /// Optional number of days to retain the message payload. Defaults to 90.
    /// Note that this is mutually exclusive with `payloadRetentionHours`.
    #[serde(rename = "payloadRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_retention_period: Option<i32>,

    /// List of free-form tags that can be filtered by when listing messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Extra parameters to pass to Transformations (for future use)
    #[serde(rename = "transformationsParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations_params: Option<serde_json::Value>,
}

impl MessageIn {
    pub fn new(event_type: String, payload: serde_json::Value) -> Self {
        Self {
            application: None,
            channels: None,
            deliver_at: None,
            event_id: None,
            event_type,
            payload,
            payload_retention_hours: None,
            payload_retention_period: None,
            tags: None,
            transformations_params: None,
        }
    }
}
