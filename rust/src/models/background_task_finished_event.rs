// this file is @generated
use serde::{Deserialize, Serialize};

use super::background_task_finished_event2::BackgroundTaskFinishedEvent2;

/// Sent when a background task is finished.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTaskFinishedEvent {
    pub data: BackgroundTaskFinishedEvent2,

    pub r#type: String,
}

impl BackgroundTaskFinishedEvent {
    pub fn new(data: BackgroundTaskFinishedEvent2, r#type: String) -> Self {
        Self { data, r#type }
    }
}
