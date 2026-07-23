// this file is @generated
use serde::{Deserialize, Serialize};

use super::{connector_out::ConnectorOut, event_type_out::EventTypeOut};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EnvironmentOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "eventTypes")]
    pub event_types: Vec<EventTypeOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,

    pub connectors: Vec<ConnectorOut>,
}

impl EnvironmentOut {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        event_types: Vec<EventTypeOut>,
        connectors: Vec<ConnectorOut>,
    ) -> Self {
        Self {
            version: None,
            created_at,
            event_types,
            settings: None,
            connectors,
        }
    }
}
