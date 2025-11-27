// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    connector_out::ConnectorOut,
    event_type_out::EventTypeOut,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EnvironmentOut {
    pub connectors: Vec<ConnectorOut>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "eventTypes")]
    pub event_types: Vec<EventTypeOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl EnvironmentOut {
    pub fn new(
        connectors: Vec<ConnectorOut>,
        created_at: String,
        event_types: Vec<EventTypeOut>,
    ) -> Self {
        Self {
            connectors,
            created_at,
            event_types,
            settings: None,
            version: None,
        }
    }
}
