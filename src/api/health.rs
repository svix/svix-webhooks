// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Health<'a> {
    cfg: &'a Configuration,
}

impl<'a> Health<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Verify the server is up and running.
    pub async fn ping(&self) -> Result<PingOut> {
        crate::request::Request::new(http::Method::GET, "/api/v1/health/ping")
            .execute(self.cfg)
            .await
    }
}
