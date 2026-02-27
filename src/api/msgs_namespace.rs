// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Creates or updates a msgs namespace with the given name.
    pub async fn create(
        &self,
        create_namespace_in: CreateNamespaceIn,
    ) -> Result<CreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/namespace/create")
            .with_body(create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Gets a msgs namespace by name.
    pub async fn get(&self, get_namespace_in: GetNamespaceIn) -> Result<GetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/namespace/get")
            .with_body(get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
