// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AppUsageStatsIn {
    pub since: chrono::DateTime<chrono::Utc>,

    pub until: chrono::DateTime<chrono::Utc>,

    /// Specific app IDs or UIDs to aggregate stats for.
    ///
    /// Note that if none of the given IDs or UIDs are resolved, a 422 response
    /// will be given.
    #[serde(rename = "appIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<std::collections::BTreeSet<String>>,
}

impl AppUsageStatsIn {
    pub fn new(since: chrono::DateTime<chrono::Utc>, until: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            since,
            until,
            app_ids: None,
        }
    }
}
