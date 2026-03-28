// this file is @generated
use serde::{Deserialize, Serialize};

use super::{node_status_out::NodeStatusOut, server_state::ServerState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterStatusOut {
    /// The unique ID of this cluster.
    ///
    /// This value is populated on cluster initialization and will never change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,

    /// The name of this cluster (as defined in the config)
    ///
    /// This value is not replicated and should only be used for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// The unique ID of the node servicing this request
    pub this_node_id: String,

    /// The cluster state of the node servicing this request
    pub this_node_state: ServerState,

    /// The timestamp of the last transaction committed on this node
    pub this_node_last_committed_timestamp: jiff::Timestamp,

    /// The last snapshot taken on this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub this_node_last_snapshot_id: Option<String>,

    /// A list of all nodes known to be in the cluster
    pub nodes: Vec<NodeStatusOut>,
}

impl ClusterStatusOut {
    pub fn new(
        this_node_id: String,
        this_node_state: ServerState,
        this_node_last_committed_timestamp: jiff::Timestamp,
        nodes: Vec<NodeStatusOut>,
    ) -> Self {
        Self {
            cluster_id: None,
            cluster_name: None,
            this_node_id,
            this_node_state,
            this_node_last_committed_timestamp,
            this_node_last_snapshot_id: None,
            nodes,
        }
    }

    pub fn with_cluster_id(mut self, value: impl Into<Option<String>>) -> Self {
        self.cluster_id = value.into();
        self
    }

    pub fn with_cluster_name(mut self, value: impl Into<Option<String>>) -> Self {
        self.cluster_name = value.into();
        self
    }

    pub fn with_this_node_last_snapshot_id(mut self, value: impl Into<Option<String>>) -> Self {
        self.this_node_last_snapshot_id = value.into();
        self
    }
}
