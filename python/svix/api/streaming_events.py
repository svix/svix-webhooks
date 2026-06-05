# This file is @generated
import typing as t
from dataclasses import dataclass
from datetime import datetime

from ..models import CreateStreamEventsIn, CreateStreamEventsOut, EventStreamOut
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class StreamingEventsCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class StreamingEventsGetOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    after: t.Optional[datetime] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "after": self.after,
            }
        )


class StreamingEventsAsync(ApiBase):
    async def create(
        self,
        stream_id: str,
        create_stream_events_in: CreateStreamEventsIn,
        options: StreamingEventsCreateOptions = StreamingEventsCreateOptions(),
    ) -> CreateStreamEventsOut:
        """Creates events on the Stream."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/{stream_id}/events",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=create_stream_events_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateStreamEventsOut.model_validate(response.json())

    async def get(
        self,
        stream_id: str,
        sink_id: str,
        options: StreamingEventsGetOptions = StreamingEventsGetOptions(),
    ) -> EventStreamOut:
        """Iterate over a stream of events.

        The sink must be of type `poller` to use the poller endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/events",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EventStreamOut.model_validate(response.json())


class StreamingEvents(ApiBase):
    def create(
        self,
        stream_id: str,
        create_stream_events_in: CreateStreamEventsIn,
        options: StreamingEventsCreateOptions = StreamingEventsCreateOptions(),
    ) -> CreateStreamEventsOut:
        """Creates events on the Stream."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/{stream_id}/events",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=create_stream_events_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateStreamEventsOut.model_validate(response.json())

    def get(
        self,
        stream_id: str,
        sink_id: str,
        options: StreamingEventsGetOptions = StreamingEventsGetOptions(),
    ) -> EventStreamOut:
        """Iterate over a stream of events.

        The sink must be of type `poller` to use the poller endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/events",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EventStreamOut.model_validate(response.json())
