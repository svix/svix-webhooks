# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    Ack,
    AckMsgRangeIn,
    AckMsgRangeOut,
    AckOut,
    AppendToStreamIn,
    AppendToStreamOut,
    DlqIn,
    DlqOut,
    FetchFromStreamIn,
    FetchFromStreamOut,
    RedriveIn,
    RedriveOut,
)


class StreamAsync(ApiBase):
    async def append(
        self,
        append_to_stream_in: AppendToStreamIn,
    ) -> AppendToStreamOut:
        """Appends messages to the stream."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/append",
            body=append_to_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=AppendToStreamOut,
        )

    async def fetch(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.

        Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
        messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
        until the visibility timeout expires, or the messages are acked."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/fetch",
            body=fetch_from_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=FetchFromStreamOut,
        )

    async def fetch_locking(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, locking over the consumer group.

        This call prevents other consumers within the same consumer group from reading from the stream
        until either the visibility timeout expires, or the last message in the batch is acknowledged."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/fetch-locking",
            body=fetch_from_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=FetchFromStreamOut,
        )

    async def ack_range(
        self,
        ack_msg_range_in: AckMsgRangeIn,
    ) -> AckMsgRangeOut:
        """Acks the messages for the consumer group, allowing more messages to be consumed."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/ack-range",
            body=ack_msg_range_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=AckMsgRangeOut,
        )

    async def ack(
        self,
        ack: Ack,
    ) -> AckOut:
        """Acks a single message."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/ack",
            body=ack.model_dump(exclude_unset=True, by_alias=True),
            response_type=AckOut,
        )

    async def dlq(
        self,
        dlq_in: DlqIn,
    ) -> DlqOut:
        """Moves a message to the dead letter queue."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/dlq",
            body=dlq_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=DlqOut,
        )

    async def redrive(
        self,
        redrive_in: RedriveIn,
    ) -> RedriveOut:
        """Redrives messages from the dead letter queue back to the stream."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/stream/redrive-dlq",
            body=redrive_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=RedriveOut,
        )


class Stream(ApiBase):
    def append(
        self,
        append_to_stream_in: AppendToStreamIn,
    ) -> AppendToStreamOut:
        """Appends messages to the stream."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/append",
            body=append_to_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=AppendToStreamOut,
        )

    def fetch(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.

        Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
        messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
        until the visibility timeout expires, or the messages are acked."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/fetch",
            body=fetch_from_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=FetchFromStreamOut,
        )

    def fetch_locking(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, locking over the consumer group.

        This call prevents other consumers within the same consumer group from reading from the stream
        until either the visibility timeout expires, or the last message in the batch is acknowledged."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/fetch-locking",
            body=fetch_from_stream_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=FetchFromStreamOut,
        )

    def ack_range(
        self,
        ack_msg_range_in: AckMsgRangeIn,
    ) -> AckMsgRangeOut:
        """Acks the messages for the consumer group, allowing more messages to be consumed."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/ack-range",
            body=ack_msg_range_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=AckMsgRangeOut,
        )

    def ack(
        self,
        ack: Ack,
    ) -> AckOut:
        """Acks a single message."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/ack",
            body=ack.model_dump(exclude_unset=True, by_alias=True),
            response_type=AckOut,
        )

    def dlq(
        self,
        dlq_in: DlqIn,
    ) -> DlqOut:
        """Moves a message to the dead letter queue."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/dlq",
            body=dlq_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=DlqOut,
        )

    def redrive(
        self,
        redrive_in: RedriveIn,
    ) -> RedriveOut:
        """Redrives messages from the dead letter queue back to the stream."""
        return self._request_sync(
            method="post",
            path="/api/v1/stream/redrive-dlq",
            body=redrive_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=RedriveOut,
        )
