// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudPubSubConfigOut {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "topicId")]
    pub topic_id: String,
}
