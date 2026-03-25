// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsStream<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsStream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Receives messages from a topic using a consumer group.
    ///
    /// Each consumer in the group reads from all partitions. Messages are locked by leases for the
    /// specified duration to prevent duplicate delivery within the same consumer group.
    pub async fn receive(
        &self,
        topic: String,
        consumer_group: String,
        msg_stream_receive_in: MsgStreamReceiveIn,
    ) -> Result<MsgStreamReceiveOut> {
        let msg_stream_receive_in = MsgStreamReceiveIn_ {
            namespace: msg_stream_receive_in.namespace,
            topic,
            consumer_group,
            batch_size: msg_stream_receive_in.batch_size,
            lease_duration_ms: msg_stream_receive_in.lease_duration_ms,
            default_starting_position: msg_stream_receive_in.default_starting_position,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.stream.receive")
            .with_body(msg_stream_receive_in)
            .execute(self.cfg)
            .await
    }

    /// Commits an offset for a consumer group on a specific partition.
    ///
    /// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
    /// successfully processed offset; future receives will start after it.
    pub async fn commit(
        &self,
        topic: String,
        consumer_group: String,
        msg_stream_commit_in: MsgStreamCommitIn,
    ) -> Result<MsgStreamCommitOut> {
        let msg_stream_commit_in = MsgStreamCommitIn_ {
            namespace: msg_stream_commit_in.namespace,
            topic,
            consumer_group,
            offset: msg_stream_commit_in.offset,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.stream.commit")
            .with_body(msg_stream_commit_in)
            .execute(self.cfg)
            .await
    }

    /// Repositions a consumer group's read cursor on a topic.
    ///
    /// Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
    /// partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
    /// `"latest"` and may be used with or without a partition suffix.
    pub async fn seek(
        &self,
        topic: String,
        consumer_group: String,
        msg_stream_seek_in: MsgStreamSeekIn,
    ) -> Result<MsgStreamSeekOut> {
        let msg_stream_seek_in = MsgStreamSeekIn_ {
            namespace: msg_stream_seek_in.namespace,
            topic,
            consumer_group,
            offset: msg_stream_seek_in.offset,
            position: msg_stream_seek_in.position,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.stream.seek")
            .with_body(msg_stream_seek_in)
            .execute(self.cfg)
            .await
    }
}
