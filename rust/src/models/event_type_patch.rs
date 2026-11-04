// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Translated descriptions keyed by BCP 47 language tag.
    #[serde(rename = "descriptionTranslations")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub description_translations: JsOption<std::collections::BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub schemas: JsOption<serde_json::Value>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<std::collections::BTreeSet<String>>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub group_name: JsOption<String>,
}

impl EventTypePatch {
    pub fn new() -> Self {
        Self {
            description: None,
            description_translations: JsOption::Undefined,
            archived: None,
            deprecated: None,
            schemas: JsOption::Undefined,
            feature_flags: JsOption::Undefined,
            group_name: JsOption::Undefined,
        }
    }
}

impl Default for EventTypePatch {
    fn default() -> Self {
        Self::new()
    }
}
