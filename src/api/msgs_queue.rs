// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsQueue<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsQueue<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Receives messages from a topic as competing consumers.
    ///
    /// Messages are individually leased for the specified duration. Multiple consumers can receive
    /// different messages from the same topic concurrently. Leased messages are skipped until they
    /// are acked or their lease expires.
    pub async fn receive(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_receive_in: MsgQueueReceiveIn,
    ) -> Result<MsgQueueReceiveOut> {
        let msg_queue_receive_in = MsgQueueReceiveIn_ {
            namespace: msg_queue_receive_in.namespace,
            topic,
            consumer_group,
            batch_size: msg_queue_receive_in.batch_size,
            lease_duration: msg_queue_receive_in.lease_duration,
            batch_wait: msg_queue_receive_in.batch_wait,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.receive")
            .with_body(msg_queue_receive_in)
            .execute(self.cfg)
            .await
    }

    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    pub async fn ack(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> Result<MsgQueueAckOut> {
        let msg_queue_ack_in = MsgQueueAckIn_ {
            namespace: msg_queue_ack_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_queue_ack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.ack")
            .with_body(msg_queue_ack_in)
            .execute(self.cfg)
            .await
    }

    /// Extends the lease on in-flight messages.
    ///
    /// Consumers that need more processing time can call this before the lease expires to prevent the
    /// message from being re-delivered to another consumer.
    pub async fn extend_lease(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_extend_lease_in: MsgQueueExtendLeaseIn,
    ) -> Result<MsgQueueExtendLeaseOut> {
        let msg_queue_extend_lease_in = MsgQueueExtendLeaseIn_ {
            namespace: msg_queue_extend_lease_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_queue_extend_lease_in.msg_ids,
            lease_duration: msg_queue_extend_lease_in.lease_duration,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.extend-lease")
            .with_body(msg_queue_extend_lease_in)
            .execute(self.cfg)
            .await
    }

    /// Configures retry and DLQ behavior for a consumer group on a topic.
    ///
    /// `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
    /// the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
    pub async fn configure(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_configure_in: MsgQueueConfigureIn,
    ) -> Result<MsgQueueConfigureOut> {
        let msg_queue_configure_in = MsgQueueConfigureIn_ {
            namespace: msg_queue_configure_in.namespace,
            topic,
            consumer_group,
            retry_schedule: msg_queue_configure_in.retry_schedule,
            dlq_topic: msg_queue_configure_in.dlq_topic,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.configure")
            .with_body(msg_queue_configure_in)
            .execute(self.cfg)
            .await
    }

    /// Rejects messages, sending them to the dead-letter queue.
    ///
    /// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
    /// move them back to the queue for reprocessing.
    pub async fn nack(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> Result<MsgQueueNackOut> {
        let msg_queue_nack_in = MsgQueueNackIn_ {
            namespace: msg_queue_nack_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_queue_nack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.nack")
            .with_body(msg_queue_nack_in)
            .execute(self.cfg)
            .await
    }

    /// Moves all dead-letter queue messages back to the main queue for reprocessing.
    pub async fn redrive_dlq(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn,
    ) -> Result<MsgQueueRedriveDlqOut> {
        let msg_queue_redrive_dlq_in = MsgQueueRedriveDlqIn_ {
            namespace: msg_queue_redrive_dlq_in.namespace,
            topic,
            consumer_group,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.queue.redrive-dlq")
            .with_body(msg_queue_redrive_dlq_in)
            .execute(self.cfg)
            .await
    }
}
