// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterRemoveNodeOut {
    pub node_id: String,
}

impl ClusterRemoveNodeOut {
    pub fn new(node_id: String) -> Self {
        Self { node_id }
    }
}
