# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgQueueAckIn,
    MsgQueueAckOut,
    MsgQueueConfigureIn,
    MsgQueueConfigureOut,
    MsgQueueNackIn,
    MsgQueueNackOut,
    MsgQueueReceiveIn,
    MsgQueueReceiveOut,
    MsgQueueRedriveDlqIn,
    MsgQueueRedriveDlqOut,
)

from ..models.msg_queue_receive_in import _MsgQueueReceiveIn
from ..models.msg_queue_ack_in import _MsgQueueAckIn
from ..models.msg_queue_configure_in import _MsgQueueConfigureIn
from ..models.msg_queue_nack_in import _MsgQueueNackIn
from ..models.msg_queue_redrive_dlq_in import _MsgQueueRedriveDlqIn


class MsgsQueueAsync(ApiBase):
    async def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            namespace=msg_queue_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_queue_receive_in.batch_size,
            lease_duration_ms=msg_queue_receive_in.lease_duration_ms,
            batch_wait_ms=msg_queue_receive_in.batch_wait_ms,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.queue.receive",
            body=body,
            response_type=MsgQueueReceiveOut,
        )

    async def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            namespace=msg_queue_ack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.queue.ack",
            body=body,
            response_type=MsgQueueAckOut,
        )

    async def configure(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_configure_in: MsgQueueConfigureIn = MsgQueueConfigureIn(),
    ) -> MsgQueueConfigureOut:
        """Configures retry and DLQ behavior for a consumer group on a topic.

        `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
        the message is moved to the DLQ (or forwarded to `dlq_topic` if set)."""
        body = _MsgQueueConfigureIn(
            namespace=msg_queue_configure_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            retry_schedule=msg_queue_configure_in.retry_schedule,
            dlq_topic=msg_queue_configure_in.dlq_topic,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.queue.configure",
            body=body,
            response_type=MsgQueueConfigureOut,
        )

    async def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> MsgQueueNackOut:
        """Rejects messages, sending them to the dead-letter queue.

        Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
        move them back to the queue for reprocessing."""
        body = _MsgQueueNackIn(
            namespace=msg_queue_nack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.queue.nack",
            body=body,
            response_type=MsgQueueNackOut,
        )

    async def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn = MsgQueueRedriveDlqIn(),
    ) -> MsgQueueRedriveDlqOut:
        """Moves all dead-letter queue messages back to the main queue for reprocessing."""
        body = _MsgQueueRedriveDlqIn(
            namespace=msg_queue_redrive_dlq_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.queue.redrive-dlq",
            body=body,
            response_type=MsgQueueRedriveDlqOut,
        )


class MsgsQueue(ApiBase):
    def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            namespace=msg_queue_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_queue_receive_in.batch_size,
            lease_duration_ms=msg_queue_receive_in.lease_duration_ms,
            batch_wait_ms=msg_queue_receive_in.batch_wait_ms,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.queue.receive",
            body=body,
            response_type=MsgQueueReceiveOut,
        )

    def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            namespace=msg_queue_ack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.queue.ack",
            body=body,
            response_type=MsgQueueAckOut,
        )

    def configure(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_configure_in: MsgQueueConfigureIn = MsgQueueConfigureIn(),
    ) -> MsgQueueConfigureOut:
        """Configures retry and DLQ behavior for a consumer group on a topic.

        `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
        the message is moved to the DLQ (or forwarded to `dlq_topic` if set)."""
        body = _MsgQueueConfigureIn(
            namespace=msg_queue_configure_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            retry_schedule=msg_queue_configure_in.retry_schedule,
            dlq_topic=msg_queue_configure_in.dlq_topic,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.queue.configure",
            body=body,
            response_type=MsgQueueConfigureOut,
        )

    def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> MsgQueueNackOut:
        """Rejects messages, sending them to the dead-letter queue.

        Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
        move them back to the queue for reprocessing."""
        body = _MsgQueueNackIn(
            namespace=msg_queue_nack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.queue.nack",
            body=body,
            response_type=MsgQueueNackOut,
        )

    def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn = MsgQueueRedriveDlqIn(),
    ) -> MsgQueueRedriveDlqOut:
        """Moves all dead-letter queue messages back to the main queue for reprocessing."""
        body = _MsgQueueRedriveDlqIn(
            namespace=msg_queue_redrive_dlq_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.queue.redrive-dlq",
            body=body,
            response_type=MsgQueueRedriveDlqOut,
        )
