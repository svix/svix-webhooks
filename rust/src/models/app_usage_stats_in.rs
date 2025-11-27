// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AppUsageStatsIn {
    /// Specific app IDs or UIDs to aggregate stats for.
    ///
    /// Note that if none of the given IDs or UIDs are resolved, a 422 response
    /// will be given.
    #[serde(rename = "appIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,

    pub since: String,

    pub until: String,
}

impl AppUsageStatsIn {
    pub fn new(
        since: String,
        until: String,
    ) -> Self {
        Self {
            app_ids: None,
            since,
            until,
        }
    }
}
