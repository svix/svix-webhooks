// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

use super::connector_kind::ConnectorKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectorPatch {
    #[serde(rename = "allowedEventTypes")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub allowed_event_types: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub logo: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<String>,
}

impl ConnectorPatch {
    pub fn new() -> Self {
        Self {
            allowed_event_types: JsOption::Undefined,
            description: None,
            feature_flags: JsOption::Undefined,
            instructions: None,
            kind: None,
            logo: JsOption::Undefined,
            name: None,
            transformation: None,
        }
    }
}
