# This file is @generated
from ..models import EmptyResponse, SinkTransformationOut, SinkTransformIn
from .common import ApiBaseAsync, ApiBaseSync


class StreamingSinkTransformationAsync(ApiBaseAsync):
    async def get(self, stream_id: str, sink_id: str) -> SinkTransformationOut:
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

    async def patch(
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


class StreamingSinkTransformation(ApiBaseSync):
    def get(self, stream_id: str, sink_id: str) -> SinkTransformationOut:
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

    def patch(
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
