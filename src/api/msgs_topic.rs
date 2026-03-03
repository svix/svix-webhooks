// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsTopic<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsTopic<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Configures the number of partitions for a topic.
    ///
    /// Partition count can only be increased, never decreased. The default for a new topic is 1.
    pub async fn configure(
        &self,
        topic: String,
        msg_topic_configure_in: MsgTopicConfigureIn,
    ) -> Result<MsgTopicConfigureOut> {
        let msg_topic_configure_in = MsgTopicConfigureIn_ {
            topic,
            partitions: msg_topic_configure_in.partitions,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/topic/configure")
            .with_body(msg_topic_configure_in)
            .execute(self.cfg)
            .await
    }
}
