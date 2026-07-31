// this file is @generated
use crate::{error::Result, Configuration};

pub struct Health<'a> {
    cfg: &'a Configuration,
}

impl<'a> Health<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Verify the API server is up and running.
    pub async fn get(&self) -> Result<()> {
        crate::request::Request::new(http::Method::GET, "/api/v1/health")
            .returns_nothing()
            .execute(self.cfg)
            .await
    }
}
