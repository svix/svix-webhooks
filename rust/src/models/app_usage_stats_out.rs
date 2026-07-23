// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AppUsageStatsOut {
    /// Any app IDs or UIDs received in the request that weren't found.
    ///
    /// Stats will be produced for all the others.
    #[serde(rename = "unresolvedAppIds")]
    pub unresolved_app_ids: std::collections::BTreeSet<String>,

    /// The QueueBackgroundTask's ID.
    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AppUsageStatsOut {
    pub fn new(
        unresolved_app_ids: std::collections::BTreeSet<String>,
        id: String,
        status: BackgroundTaskStatus,
        task: BackgroundTaskType,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            unresolved_app_ids,
            id,
            status,
            task,
            updated_at,
        }
    }
}
