// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

use super::connector_kind::ConnectorKind;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConnectorPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub logo: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(rename = "allowedEventTypes")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub allowed_event_types: JsOption<std::collections::BTreeSet<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<std::collections::BTreeSet<String>>,
}

impl ConnectorPatch {
    pub fn new() -> Self {
        Self {
            name: None,
            logo: JsOption::Undefined,
            description: None,
            kind: None,
            instructions: None,
            allowed_event_types: JsOption::Undefined,
            transformation: None,
            feature_flags: JsOption::Undefined,
        }
    }
}

impl Default for ConnectorPatch {
    fn default() -> Self {
        Self::new()
    }
}
