// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudPubSubConfigPatch {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}

impl GoogleCloudPubSubConfigPatch {
    pub fn new() -> Self {
        Self {
            project_id: None,
            topic_id: None,
            credentials: None,
        }
    }
}

impl Default for GoogleCloudPubSubConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
