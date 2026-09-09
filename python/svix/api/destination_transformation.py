# This file is @generated
from ..models import DestinationTransformationOut, DestinationTransformIn, EmptyResponse
from .common import ApiBaseAsync, ApiBaseSync


class DestinationTransformationAsync(ApiBaseAsync):
    async def get(
        self, app_id: str, destination_id: str
    ) -> DestinationTransformationOut:
        """Get the transformation code associated with this destination."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/destination/{destination_id}/transformation",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )
        return DestinationTransformationOut.model_validate(response.json())

    async def patch(
        self,
        app_id: str,
        destination_id: str,
        destination_transform_in: DestinationTransformIn,
    ) -> EmptyResponse:
        """Set or unset the transformation code associated with this destination."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/destination/{destination_id}/transformation",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_transform_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())


class DestinationTransformation(ApiBaseSync):
    def get(self, app_id: str, destination_id: str) -> DestinationTransformationOut:
        """Get the transformation code associated with this destination."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/destination/{destination_id}/transformation",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )
        return DestinationTransformationOut.model_validate(response.json())

    def patch(
        self,
        app_id: str,
        destination_id: str,
        destination_transform_in: DestinationTransformIn,
    ) -> EmptyResponse:
        """Set or unset the transformation code associated with this destination."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/destination/{destination_id}/transformation",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_transform_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EmptyResponse.model_validate(response.json())
