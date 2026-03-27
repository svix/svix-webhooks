// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterForceSnapshotOut {
    pub snapshot_time: jiff::Timestamp,

    pub snapshot_log_index: u64,
}

impl ClusterForceSnapshotOut {
    pub fn new(snapshot_time: jiff::Timestamp, snapshot_log_index: u64) -> Self {
        Self {
            snapshot_time,
            snapshot_log_index,
        }
    }
}
