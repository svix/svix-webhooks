// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Stream<'a> {
    cfg: &'a Configuration,
}

impl<'a> Stream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Appends messages to the stream.
    pub async fn append(&self, append_to_stream_in: AppendToStreamIn) -> Result<AppendToStreamOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/append")
            .with_body(append_to_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
    ///
    /// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
    /// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
    /// until the visibility timeout expires, or the messages are acked.
    pub async fn fetch(
        &self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> Result<FetchFromStreamOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/fetch")
            .with_body(fetch_from_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Fetches messages from the stream, locking over the consumer group.
    ///
    /// This call prevents other consumers within the same consumer group from reading from the stream
    /// until either the visibility timeout expires, or the last message in the batch is acknowledged.
    pub async fn fetch_locking(
        &self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> Result<FetchFromStreamOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/fetch-locking")
            .with_body(fetch_from_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Acks the messages for the consumer group, allowing more messages to be consumed.
    pub async fn ack_range(&self, ack_msg_range_in: AckMsgRangeIn) -> Result<AckMsgRangeOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/ack-range")
            .with_body(ack_msg_range_in)
            .execute(self.cfg)
            .await
    }

    /// Acks a single message.
    pub async fn ack(&self, ack: Ack) -> Result<AckOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/ack")
            .with_body(ack)
            .execute(self.cfg)
            .await
    }

    /// Moves a message to the dead letter queue.
    pub async fn dlq(&self, dlq_in: DlqIn) -> Result<DlqOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/dlq")
            .with_body(dlq_in)
            .execute(self.cfg)
            .await
    }

    /// Redrives messages from the dead letter queue back to the stream.
    pub async fn redrive(&self, redrive_in: RedriveIn) -> Result<RedriveOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/stream/redrive-dlq")
            .with_body(redrive_in)
            .execute(self.cfg)
            .await
    }
}
