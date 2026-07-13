// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApplicationPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Maximum messages per second to send to this application.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub throttle_rate: JsOption<u16>,

    /// The Application's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl ApplicationPatch {
    pub fn new() -> Self {
        Self {
            name: None,
            throttle_rate: JsOption::Undefined,
            uid: JsOption::Undefined,
            metadata: None,
        }
    }
}

impl Default for ApplicationPatch {
    fn default() -> Self {
        Self::new()
    }
}
