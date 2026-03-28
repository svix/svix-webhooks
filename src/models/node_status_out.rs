// this file is @generated
use serde::{Deserialize, Serialize};

use super::server_state::ServerState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeStatusOut {
    /// A unique ID representing this node.
    ///
    /// This will never change unless the node is erased and reset
    pub node_id: String,

    /// The advertised inter-server (cluster) address of this node.
    pub address: String,

    /// The last known state of this node
    pub state: ServerState,

    /// The index of the last log applied on this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_committed_log_index: Option<u64>,

    /// The raft term of the last committed leadership
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_committed_term: Option<u64>,
}

impl NodeStatusOut {
    pub fn new(node_id: String, address: String, state: ServerState) -> Self {
        Self {
            node_id,
            address,
            state,
            last_committed_log_index: None,
            last_committed_term: None,
        }
    }

    pub fn with_last_committed_log_index(mut self, value: impl Into<Option<u64>>) -> Self {
        self.last_committed_log_index = value.into();
        self
    }

    pub fn with_last_committed_term(mut self, value: impl Into<Option<u64>>) -> Self {
        self.last_committed_term = value.into();
        self
    }
}
