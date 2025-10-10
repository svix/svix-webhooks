# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import ListResponseStreamOut, StreamIn, StreamOut, StreamPatch
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class StreamingStreamListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
            }
        )


@dataclass
class StreamingStreamCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class StreamingStreamAsync(ApiBase):
    async def list(
        self, options: StreamingStreamListOptions = StreamingStreamListOptions()
    ) -> ListResponseStreamOut:
        """List of all the organization's streams."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamOut.model_validate(response.json())

    async def create(
        self,
        stream_in: StreamIn,
        options: StreamingStreamCreateOptions = StreamingStreamCreateOptions(),
    ) -> StreamOut:
        """Creates a new stream."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())

    async def get(self, stream_id: str) -> StreamOut:
        """Get a stream by id or uid."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
        )
        return StreamOut.model_validate(response.json())

    async def update(self, stream_id: str, stream_in: StreamIn) -> StreamOut:
        """Update a stream."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
            json_body=stream_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())

    async def delete(self, stream_id: str) -> None:
        """Delete a stream."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
        )

    async def patch(self, stream_id: str, stream_patch: StreamPatch) -> StreamOut:
        """Partially update a stream."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
            json_body=stream_patch.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())


class StreamingStream(ApiBase):
    def list(
        self, options: StreamingStreamListOptions = StreamingStreamListOptions()
    ) -> ListResponseStreamOut:
        """List of all the organization's streams."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamOut.model_validate(response.json())

    def create(
        self,
        stream_in: StreamIn,
        options: StreamingStreamCreateOptions = StreamingStreamCreateOptions(),
    ) -> StreamOut:
        """Creates a new stream."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())

    def get(self, stream_id: str) -> StreamOut:
        """Get a stream by id or uid."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
        )
        return StreamOut.model_validate(response.json())

    def update(self, stream_id: str, stream_in: StreamIn) -> StreamOut:
        """Update a stream."""
        response = self._request_sync(
            method="put",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
            json_body=stream_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())

    def delete(self, stream_id: str) -> None:
        """Delete a stream."""
        self._request_sync(
            method="delete",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
        )

    def patch(self, stream_id: str, stream_patch: StreamPatch) -> StreamOut:
        """Partially update a stream."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/stream/{stream_id}",
            path_params={
                "stream_id": stream_id,
            },
            json_body=stream_patch.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamOut.model_validate(response.json())
