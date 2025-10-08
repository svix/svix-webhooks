# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    ListResponseStreamEventTypeOut,
    StreamEventTypeIn,
    StreamEventTypeOut,
    StreamEventTypePatch,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class StreamEventTypeListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""
    include_archived: t.Optional[bool] = None
    """Include archived (deleted but not expunged) items in the response."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
                "include_archived": self.include_archived,
            }
        )


@dataclass
class StreamEventTypeCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class StreamEventTypeDeleteOptions(BaseOptions):
    expunge: t.Optional[bool] = None
    """By default, event types are archived when "deleted". With this flag, they are deleted entirely."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "expunge": self.expunge,
            }
        )


class StreamEventTypeAsync(ApiBase):
    async def list(
        self, options: StreamEventTypeListOptions = StreamEventTypeListOptions()
    ) -> ListResponseStreamEventTypeOut:
        """List of all the organization's event types for streaming."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamEventTypeOut.model_validate(response.json())

    async def create(
        self,
        stream_event_type_in: StreamEventTypeIn,
        options: StreamEventTypeCreateOptions = StreamEventTypeCreateOptions(),
    ) -> StreamEventTypeOut:
        """Create an event type for Streams."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_event_type_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())

    async def get(self, name: str) -> StreamEventTypeOut:
        """Get an event type."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
        )
        return StreamEventTypeOut.model_validate(response.json())

    async def update(
        self, name: str, stream_event_type_in: StreamEventTypeIn
    ) -> StreamEventTypeOut:
        """Update or create a event type for Streams."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            json_body=stream_event_type_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())

    async def delete(
        self,
        name: str,
        options: StreamEventTypeDeleteOptions = StreamEventTypeDeleteOptions(),
    ) -> None:
        """Delete an event type."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )

    async def patch(
        self, name: str, stream_event_type_patch: StreamEventTypePatch
    ) -> StreamEventTypeOut:
        """Patch an event type for Streams."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            json_body=stream_event_type_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())


class StreamEventType(ApiBase):
    def list(
        self, options: StreamEventTypeListOptions = StreamEventTypeListOptions()
    ) -> ListResponseStreamEventTypeOut:
        """List of all the organization's event types for streaming."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamEventTypeOut.model_validate(response.json())

    def create(
        self,
        stream_event_type_in: StreamEventTypeIn,
        options: StreamEventTypeCreateOptions = StreamEventTypeCreateOptions(),
    ) -> StreamEventTypeOut:
        """Create an event type for Streams."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_event_type_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())

    def get(self, name: str) -> StreamEventTypeOut:
        """Get an event type."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
        )
        return StreamEventTypeOut.model_validate(response.json())

    def update(
        self, name: str, stream_event_type_in: StreamEventTypeIn
    ) -> StreamEventTypeOut:
        """Update or create a event type for Streams."""
        response = self._request_sync(
            method="put",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            json_body=stream_event_type_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())

    def delete(
        self,
        name: str,
        options: StreamEventTypeDeleteOptions = StreamEventTypeDeleteOptions(),
    ) -> None:
        """Delete an event type."""
        self._request_sync(
            method="delete",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )

    def patch(
        self, name: str, stream_event_type_patch: StreamEventTypePatch
    ) -> StreamEventTypeOut:
        """Patch an event type for Streams."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/stream/event-type/{name}",
            path_params={
                "name": name,
            },
            json_body=stream_event_type_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamEventTypeOut.model_validate(response.json())
