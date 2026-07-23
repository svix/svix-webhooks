// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTaskOut {
    pub data: serde_json::Value,

    /// The QueueBackgroundTask's ID.
    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl BackgroundTaskOut {
    pub fn new(
        data: serde_json::Value,
        id: String,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            data,
            id,
            status,
            task,
            updated_at,
        }
    }
}
