// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterForceSnapshotOut {
    /// The wall-clock time at which the snapshot was initiated
    pub snapshot_time: jiff::Timestamp,

    /// The log index at which the snapshot was initiated
    pub snapshot_log_index: u64,

    /// If this is `null`, the snapshot is still building in the background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

impl ClusterForceSnapshotOut {
    pub fn new(snapshot_time: jiff::Timestamp, snapshot_log_index: u64) -> Self {
        Self {
            snapshot_time,
            snapshot_log_index,
            snapshot_id: None,
        }
    }

    pub fn with_snapshot_id(mut self, value: impl Into<Option<String>>) -> Self {
        self.snapshot_id = value.into();
        self
    }
}
