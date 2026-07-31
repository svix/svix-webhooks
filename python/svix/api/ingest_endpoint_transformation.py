# This file is @generated
from ..models import IngestEndpointTransformationOut, IngestEndpointTransformationPatch
from .common import ApiBaseAsync, ApiBaseSync


class IngestEndpointTransformationAsync(ApiBaseAsync):
    async def transformation(
        self, source_id: str, endpoint_id: str
    ) -> IngestEndpointTransformationOut:
        """Get the transformation code associated with this ingest endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointTransformationOut.model_validate(response.json())

    async def patch(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_transformation_patch: IngestEndpointTransformationPatch,
    ) -> None:
        """Set or unset the transformation code associated with this ingest endpoint."""
        await self._request_asyncio(
            method="patch",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_transformation_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class IngestEndpointTransformation(ApiBaseSync):
    def transformation(
        self, source_id: str, endpoint_id: str
    ) -> IngestEndpointTransformationOut:
        """Get the transformation code associated with this ingest endpoint."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointTransformationOut.model_validate(response.json())

    def patch(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_transformation_patch: IngestEndpointTransformationPatch,
    ) -> None:
        """Set or unset the transformation code associated with this ingest endpoint."""
        self._request_sync(
            method="patch",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_transformation_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
