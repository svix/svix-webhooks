// this file is @generated
use super::{MsgsNamespace, MsgsStream, MsgsTopic};
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

    pub fn stream(&self) -> MsgsStream<'a> {
        MsgsStream::new(self.cfg)
    }

    pub fn topic(&self) -> MsgsTopic<'a> {
        MsgsTopic::new(self.cfg)
    }

    /// Publishes messages to a topic within a namespace.
    pub async fn publish(
        &self,
        topic: String,
        msg_publish_in: MsgPublishIn,
    ) -> Result<MsgPublishOut> {
        let msg_publish_in = MsgPublishIn_ {
            topic,
            msgs: msg_publish_in.msgs,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/publish")
            .with_body(msg_publish_in)
            .execute(self.cfg)
            .await
    }
}
