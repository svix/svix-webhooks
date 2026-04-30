// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Deprecated, use `throttleRate` instead.
    #[deprecated]
    #[serde(rename = "rateLimit")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub rate_limit: JsOption<u16>,

    /// Maximum messages per second to send to this application.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub throttle_rate: JsOption<u16>,

    /// The Application's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,
}

impl ApplicationPatch {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            metadata: None,
            name: None,
            rate_limit: JsOption::Undefined,
            throttle_rate: JsOption::Undefined,
            uid: JsOption::Undefined,
        }
    }
}
