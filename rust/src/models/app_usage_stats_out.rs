// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    background_task_status::BackgroundTaskStatus,
    background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AppUsageStatsOut {
    /// The QueueBackgroundTask's ID.
    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,

    /// Any app IDs or UIDs received in the request that weren't found.
    ///
    /// Stats will be produced for all the others.
    #[serde(rename = "unresolvedAppIds")]
    pub unresolved_app_ids: Vec<String>,
}

impl AppUsageStatsOut {
    pub fn new(
        id: String,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
        unresolved_app_ids: Vec<String>,
    ) -> Self {
        Self {
            id,
            status,
            task,
            unresolved_app_ids,
        }
    }
}
