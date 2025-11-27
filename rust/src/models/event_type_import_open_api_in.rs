// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

/// Import a list of event types from webhooks defined in an OpenAPI spec.
///
/// The OpenAPI spec can be specified as either `spec` given the spec as a JSON
/// object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by
/// the server. Sending neither or both is invalid, resulting in a `400` **Bad
/// Request**.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeImportOpenApiIn {
    /// If `true`, return the event types that would be modified without
    /// actually modifying them.
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// If `true`, all existing event types that are not in the spec will be
    /// archived.
    #[serde(rename = "replaceAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_all: Option<bool>,

    /// A pre-parsed JSON spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<serde_json::Value>,

    /// A string, parsed by the server as YAML or JSON.
    #[serde(rename = "specRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_raw: Option<String>,
}

impl EventTypeImportOpenApiIn {
    pub fn new() -> Self {
        Self {
            dry_run: None,
            replace_all: None,
            spec: None,
            spec_raw: None,
        }
    }
}
