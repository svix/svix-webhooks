// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AdminCluster<'a> {
    cfg: &'a Configuration,
}

impl<'a> AdminCluster<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get information about the current cluster
    pub async fn status(&self) -> Result<ClusterStatusOut> {
        crate::request::Request::new(http::Method::GET, "/api/v1.admin.cluster.status")
            .execute(self.cfg)
            .await
    }

    /// Initialize this node as the leader of a new cluster
    ///
    /// This operation may only be performed against a node which has not been
    /// initialized and is not currently a member of a cluster.
    pub async fn initialize(
        &self,
        cluster_initialize_in: ClusterInitializeIn,
    ) -> Result<ClusterInitializeOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.cluster.initialize")
            .with_body(cluster_initialize_in)
            .execute(self.cfg)
            .await
    }

    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    pub async fn remove_node(
        &self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> Result<ClusterRemoveNodeOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.cluster.remove-node")
            .with_body(cluster_remove_node_in)
            .execute(self.cfg)
            .await
    }

    /// Force the cluster to take a snapshot immediately
    pub async fn force_snapshot(
        &self,
        cluster_force_snapshot_in: ClusterForceSnapshotIn,
    ) -> Result<ClusterForceSnapshotOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.cluster.force-snapshot")
            .with_body(cluster_force_snapshot_in)
            .execute(self.cfg)
            .await
    }
}
