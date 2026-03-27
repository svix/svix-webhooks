// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterInitializeOut {
    pub cluster_id: String,
}

impl ClusterInitializeOut {
    pub fn new(cluster_id: String) -> Self {
        Self { cluster_id }
    }
}
