// this file is @generated (with minor manual edits)
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTaskOut {
    pub data: serde_json::Value,

    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,
}

impl BackgroundTaskOut {
    pub fn new(
        data: serde_json::Value,
        id: String,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
    ) -> Self {
        Self {
            data,
            id,
            status,
            task,
        }
    }
}
