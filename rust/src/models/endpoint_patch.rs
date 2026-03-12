// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointPatch {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub channels: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "filterTypes")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub filter_types: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "rateLimit")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub rate_limit: JsOption<u16>,

    /// The endpoint's verification secret.
    ///
    /// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    /// It is recommended to not set this and let the server generate the
    /// secret.
    #[deprecated]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub secret: JsOption<String>,

    /// The Endpoint's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u16>,
}

impl EndpointPatch {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            channels: JsOption::Undefined,
            description: None,
            disabled: None,
            filter_types: JsOption::Undefined,
            metadata: None,
            rate_limit: JsOption::Undefined,
            secret: JsOption::Undefined,
            uid: JsOption::Undefined,
            url: None,
            version: None,
        }
    }
}
