// this file is @generated
use super::AdminCluster;
use crate::{Configuration, error::Result, models::*};

pub struct Admin<'a> {
    cfg: &'a Configuration,
}

impl<'a> Admin<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn cluster(&self) -> AdminCluster<'a> {
        AdminCluster::new(self.cfg)
    }

    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    pub async fn cluster_remove_node(
        &self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> Result<ClusterRemoveNodeOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/admin/cluster/remove-node")
            .with_body(cluster_remove_node_in)
            .execute(self.cfg)
            .await
    }
}
