# This file is @generated
from ..models import EndpointHeadersOut, HttpSinkHeadersPatchIn, SinkTransformationOut
from .common import ApiBase
from .stream_event_type import (
    StreamEventType,
    StreamEventTypeAsync,
)
from .stream_events import (
    StreamEvents,
    StreamEventsAsync,
)
from .stream_sink import (
    StreamSink,
    StreamSinkAsync,
)
from .stream_stream import (
    StreamStream,
    StreamStreamAsync,
)


class StreamAsync(ApiBase):
    @property
    def event_type(self) -> StreamEventTypeAsync:
        return StreamEventTypeAsync(self._client)

    @property
    def events(self) -> StreamEventsAsync:
        return StreamEventsAsync(self._client)

    @property
    def sink(self) -> StreamSinkAsync:
        return StreamSinkAsync(self._client)

    @property
    def stream(self) -> StreamStreamAsync:
        return StreamStreamAsync(self._client)

    async def sink_headers_get(
        self, stream_id: str, sink_id: str
    ) -> EndpointHeadersOut:
        """Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return EndpointHeadersOut.model_validate(response.json())

    async def sink_headers_patch(
        self,
        stream_id: str,
        sink_id: str,
        http_sink_headers_patch_in: HttpSinkHeadersPatchIn,
    ) -> EndpointHeadersOut:
        """Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=http_sink_headers_patch_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EndpointHeadersOut.model_validate(response.json())

    async def sink_transformation_get(
        self, stream_id: str, sink_id: str
    ) -> SinkTransformationOut:
        """Get the transformation code associated with this sink."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return SinkTransformationOut.model_validate(response.json())


class Stream(ApiBase):
    @property
    def event_type(self) -> StreamEventType:
        return StreamEventType(self._client)

    @property
    def events(self) -> StreamEvents:
        return StreamEvents(self._client)

    @property
    def sink(self) -> StreamSink:
        return StreamSink(self._client)

    @property
    def stream(self) -> StreamStream:
        return StreamStream(self._client)

    def sink_headers_get(self, stream_id: str, sink_id: str) -> EndpointHeadersOut:
        """Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return EndpointHeadersOut.model_validate(response.json())

    def sink_headers_patch(
        self,
        stream_id: str,
        sink_id: str,
        http_sink_headers_patch_in: HttpSinkHeadersPatchIn,
    ) -> EndpointHeadersOut:
        """Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
            json_body=http_sink_headers_patch_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EndpointHeadersOut.model_validate(response.json())

    def sink_transformation_get(
        self, stream_id: str, sink_id: str
    ) -> SinkTransformationOut:
        """Get the transformation code associated with this sink."""
        response = self._request_sync(
            method="get",
            path="/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
            path_params={
                "stream_id": stream_id,
                "sink_id": sink_id,
            },
        )
        return SinkTransformationOut.model_validate(response.json())
