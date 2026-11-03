// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct McpEventsSubscribeOut {
    /// The subscription's ID, which is also the UID of its endpoint.
    pub id: String,

    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// When the subscription expires, or `null` if it never does.
    #[serde(rename = "refreshBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_before: Option<chrono::DateTime<chrono::Utc>>,

    /// Always `null`, since events can't be replayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Always `false`, since events can't be replayed.
    pub truncated: bool,
}
