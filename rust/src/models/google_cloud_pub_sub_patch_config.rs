// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudPubSubPatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,

    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

impl GoogleCloudPubSubPatchConfig {
    pub fn new() -> Self {
        Self {
            credentials: None,
            project_id: None,
            topic_id: None,
        }
    }
}
