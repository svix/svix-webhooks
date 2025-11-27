// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    connector_in::ConnectorIn,
    event_type_in::EventTypeIn,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EnvironmentIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ConnectorIn>>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeIn>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

impl EnvironmentIn {
    pub fn new() -> Self {
        Self {
            connectors: None,
            event_types: None,
            settings: None,
        }
    }
}
