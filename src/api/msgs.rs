// this file is @generated
use super::MsgsNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct Msgs<'a> {
    cfg: &'a Configuration,
}

impl<'a> Msgs<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> MsgsNamespace<'a> {
        MsgsNamespace::new(self.cfg)
    }

    /// Publishes messages to a topic within a namespace.
    pub async fn publish(&self, publish_in: PublishIn) -> Result<PublishOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/publish")
            .with_body(publish_in)
            .execute(self.cfg)
            .await
    }
}
