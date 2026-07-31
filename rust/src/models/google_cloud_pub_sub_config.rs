// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudPubSubConfig {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "topicId")]
    pub topic_id: String,

    /// Google Cloud Credentials JSON Object as a string.
    pub credentials: String,
}

impl GoogleCloudPubSubConfig {
    pub fn new(project_id: String, topic_id: String, credentials: String) -> Self {
        Self {
            project_id,
            topic_id,
            credentials,
        }
    }
}
