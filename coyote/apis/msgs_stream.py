# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgStreamCommitIn,
    MsgStreamCommitOut,
    MsgStreamReceiveIn,
    MsgStreamReceiveOut,
)

from ..models.msg_stream_receive_in import _MsgStreamReceiveIn
from ..models.msg_stream_commit_in import _MsgStreamCommitIn


class MsgsStreamAsync(ApiBase):
    async def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_receive_in: MsgStreamReceiveIn = MsgStreamReceiveIn(),
    ) -> MsgStreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        body = _MsgStreamReceiveIn(
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_stream_receive_in.batch_size,
            lease_duration_millis=msg_stream_receive_in.lease_duration_millis,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=body,
            response_type=MsgStreamReceiveOut,
        )

    async def commit(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_commit_in: MsgStreamCommitIn,
    ) -> MsgStreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        body = _MsgStreamCommitIn(
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_commit_in.offset,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )


class MsgsStream(ApiBase):
    def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_receive_in: MsgStreamReceiveIn = MsgStreamReceiveIn(),
    ) -> MsgStreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        body = _MsgStreamReceiveIn(
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_stream_receive_in.batch_size,
            lease_duration_millis=msg_stream_receive_in.lease_duration_millis,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=body,
            response_type=MsgStreamReceiveOut,
        )

    def commit(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_commit_in: MsgStreamCommitIn,
    ) -> MsgStreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        body = _MsgStreamCommitIn(
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_commit_in.offset,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )
