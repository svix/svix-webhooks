// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AggregateEventTypesOut {
    /// The QueueBackgroundTask's ID.
    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl AggregateEventTypesOut {
    pub fn new(
        id: String,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            status,
            task,
            updated_at,
        }
    }
}
