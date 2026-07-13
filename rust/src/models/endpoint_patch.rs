// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Maximum messages per second to send to this endpoint.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub throttle_rate: JsOption<u16>,

    /// The Endpoint's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "filterTypes")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub filter_types: JsOption<Vec<String>>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub channels: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl EndpointPatch {
    pub fn new() -> Self {
        Self {
            description: None,
            throttle_rate: JsOption::Undefined,
            uid: JsOption::Undefined,
            url: None,
            disabled: None,
            filter_types: JsOption::Undefined,
            channels: JsOption::Undefined,
            metadata: None,
        }
    }
}

impl Default for EndpointPatch {
    fn default() -> Self {
        Self::new()
    }
}
