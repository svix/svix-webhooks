# This file is @generated
from ..models import EndpointTransformationOut, EndpointTransformationPatch
from .common import ApiBaseAsync, ApiBaseSync


class EndpointTransformationAsync(ApiBaseAsync):
    async def get(self, app_id: str, endpoint_id: str) -> EndpointTransformationOut:
        """Get the transformation code associated with this endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointTransformationOut.model_validate(response.json())

    async def patch(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_patch: EndpointTransformationPatch,
    ) -> None:
        """Set or unset the transformation code associated with this endpoint."""
        await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class EndpointTransformation(ApiBaseSync):
    def get(self, app_id: str, endpoint_id: str) -> EndpointTransformationOut:
        """Get the transformation code associated with this endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointTransformationOut.model_validate(response.json())

    def patch(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_patch: EndpointTransformationPatch,
    ) -> None:
        """Set or unset the transformation code associated with this endpoint."""
        self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
