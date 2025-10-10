# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    EmptyResponse,
    EndpointSecretRotateIn,
    ListResponseStreamSinkOut,
    SinkSecretOut,
    SinkTransformIn,
    StreamSinkIn,
    StreamSinkOut,
    StreamSinkPatch,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class StreamingSinkListOptions(BaseOptions):
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
class StreamingSinkCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class StreamingSinkRotateSecretOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class StreamingSinkAsync(ApiBase):
    async def list(
        self,
        stream_id: str,
        options: StreamingSinkListOptions = StreamingSinkListOptions(),
    ) -> ListResponseStreamSinkOut:
        """List of all the stream's sinks."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamSinkOut.model_validate(response.json())

    async def create(
        self,
        stream_id: str,
        stream_sink_in: StreamSinkIn,
        options: StreamingSinkCreateOptions = StreamingSinkCreateOptions(),
    ) -> StreamSinkOut:
        """Creates a new sink."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/{stream_id}/sink",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_sink_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamSinkOut.model_validate(response.json())

    async def get(self, stream_id: str, sink_id: str) -> StreamSinkOut:
        """Get a sink by id or uid."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return StreamSinkOut.model_validate(response.json())

    async def update(
        self, stream_id: str, sink_id: str, stream_sink_in: StreamSinkIn
    ) -> StreamSinkOut:
        """Update a sink."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=stream_sink_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamSinkOut.model_validate(response.json())

    async def delete(self, stream_id: str, sink_id: str) -> None:
        """Delete a sink."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )

    async def patch(
        self, stream_id: str, sink_id: str, stream_sink_patch: StreamSinkPatch
    ) -> StreamSinkOut:
        """Partially update a sink."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=stream_sink_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamSinkOut.model_validate(response.json())

    async def get_secret(self, stream_id: str, sink_id: str) -> SinkSecretOut:
        """Get the sink's signing secret (only supported for http sinks)

        This is used to verify the authenticity of the delivery.

        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return SinkSecretOut.model_validate(response.json())

    async def rotate_secret(
        self,
        stream_id: str,
        sink_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: StreamingSinkRotateSecretOptions = StreamingSinkRotateSecretOptions(),
    ) -> EmptyResponse:
        """Rotates the signing secret (only supported for http sinks)."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_secret_rotate_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())

    async def transformation_partial_update(
        self, stream_id: str, sink_id: str, sink_transform_in: SinkTransformIn
    ) -> EmptyResponse:
        """Set or unset the transformation code associated with this sink."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=sink_transform_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())


class StreamingSink(ApiBase):
    def list(
        self,
        stream_id: str,
        options: StreamingSinkListOptions = StreamingSinkListOptions(),
    ) -> ListResponseStreamSinkOut:
        """List of all the stream's sinks."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseStreamSinkOut.model_validate(response.json())

    def create(
        self,
        stream_id: str,
        stream_sink_in: StreamSinkIn,
        options: StreamingSinkCreateOptions = StreamingSinkCreateOptions(),
    ) -> StreamSinkOut:
        """Creates a new sink."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/{stream_id}/sink",
            path_params={
                "stream_id": stream_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=stream_sink_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamSinkOut.model_validate(response.json())

    def get(self, stream_id: str, sink_id: str) -> StreamSinkOut:
        """Get a sink by id or uid."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return StreamSinkOut.model_validate(response.json())

    def update(
        self, stream_id: str, sink_id: str, stream_sink_in: StreamSinkIn
    ) -> StreamSinkOut:
        """Update a sink."""
        response = self._request_sync(
            method="put",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=stream_sink_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return StreamSinkOut.model_validate(response.json())

    def delete(self, stream_id: str, sink_id: str) -> None:
        """Delete a sink."""
        self._request_sync(
            method="delete",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )

    def patch(
        self, stream_id: str, sink_id: str, stream_sink_patch: StreamSinkPatch
    ) -> StreamSinkOut:
        """Partially update a sink."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=stream_sink_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return StreamSinkOut.model_validate(response.json())

    def get_secret(self, stream_id: str, sink_id: str) -> SinkSecretOut:
        """Get the sink's signing secret (only supported for http sinks)

        This is used to verify the authenticity of the delivery.

        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return SinkSecretOut.model_validate(response.json())

    def rotate_secret(
        self,
        stream_id: str,
        sink_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: StreamingSinkRotateSecretOptions = StreamingSinkRotateSecretOptions(),
    ) -> EmptyResponse:
        """Rotates the signing secret (only supported for http sinks)."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_secret_rotate_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())

    def transformation_partial_update(
        self, stream_id: str, sink_id: str, sink_transform_in: SinkTransformIn
    ) -> EmptyResponse:
        """Set or unset the transformation code associated with this sink."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=sink_transform_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())
