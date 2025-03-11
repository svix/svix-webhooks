// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTaskFinishedEvent2 {
    pub data: serde_json::Value,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,

    /// The QueueBackgroundTask's ID.
    #[serde(rename = "taskId")]
    pub task_id: String,
}

impl BackgroundTaskFinishedEvent2 {
    pub fn new(
        data: serde_json::Value,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
        task_id: String,
    ) -> Self {
        Self {
            data,
            status,
            task,
            task_id,
        }
    }
}
