# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgStreamCommitIn,
    MsgStreamCommitOut,
    MsgStreamReceiveIn,
    MsgStreamReceiveOut,
    MsgStreamSeekIn,
    MsgStreamSeekOut,
)

from ..models.msg_stream_receive_in import _MsgStreamReceiveIn
from ..models.msg_stream_commit_in import _MsgStreamCommitIn
from ..models.msg_stream_seek_in import _MsgStreamSeekIn


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
            namespace=msg_stream_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_stream_receive_in.batch_size,
            lease_duration_ms=msg_stream_receive_in.lease_duration_ms,
            default_starting_position=msg_stream_receive_in.default_starting_position,
            batch_wait_ms=msg_stream_receive_in.batch_wait_ms,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.stream.receive",
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
            namespace=msg_stream_commit_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_commit_in.offset,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.stream.commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )

    async def seek(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_seek_in: MsgStreamSeekIn = MsgStreamSeekIn(),
    ) -> MsgStreamSeekOut:
        """Repositions a consumer group's read cursor on a topic.

        Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
        partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
        `"latest"` and may be used with or without a partition suffix."""
        body = _MsgStreamSeekIn(
            namespace=msg_stream_seek_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_seek_in.offset,
            position=msg_stream_seek_in.position,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.stream.seek",
            body=body,
            response_type=MsgStreamSeekOut,
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
            namespace=msg_stream_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_stream_receive_in.batch_size,
            lease_duration_ms=msg_stream_receive_in.lease_duration_ms,
            default_starting_position=msg_stream_receive_in.default_starting_position,
            batch_wait_ms=msg_stream_receive_in.batch_wait_ms,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.stream.receive",
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
            namespace=msg_stream_commit_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_commit_in.offset,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.stream.commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )

    def seek(
        self,
        topic: str,
        consumer_group: str,
        msg_stream_seek_in: MsgStreamSeekIn = MsgStreamSeekIn(),
    ) -> MsgStreamSeekOut:
        """Repositions a consumer group's read cursor on a topic.

        Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
        partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
        `"latest"` and may be used with or without a partition suffix."""
        body = _MsgStreamSeekIn(
            namespace=msg_stream_seek_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            offset=msg_stream_seek_in.offset,
            position=msg_stream_seek_in.position,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.stream.seek",
            body=body,
            response_type=MsgStreamSeekOut,
        )
