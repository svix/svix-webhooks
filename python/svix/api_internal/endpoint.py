# This file is @generated
from deprecated import deprecated

from ..models import EndpointTransformationIn
from .common import ApiBase
from .endpoint_auto_config import (
    EndpointAutoConfig,
    EndpointAutoConfigAsync,
)


class EndpointAsync(ApiBase):
    @property
    def auto_config(self) -> EndpointAutoConfigAsync:
        return EndpointAutoConfigAsync(self._client)

    @deprecated
    async def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """This operation was renamed to `set-transformation`."""
        await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class Endpoint(ApiBase):
    @property
    def auto_config(self) -> EndpointAutoConfig:
        return EndpointAutoConfig(self._client)

    @deprecated
    def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """This operation was renamed to `set-transformation`."""
        self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
