// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudPubSubConfig {
    /// Google Cloud Credentials JSON Object as a string.
    pub credentials: String,

    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "topicId")]
    pub topic_id: String,
}

impl GoogleCloudPubSubConfig {
    pub fn new(credentials: String, project_id: String, topic_id: String) -> Self {
        Self {
            credentials,
            project_id,
            topic_id,
        }
    }
}
