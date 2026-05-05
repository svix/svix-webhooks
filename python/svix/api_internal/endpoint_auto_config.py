# This file is @generated
from ..models import EndpointOut, SubscribeIn
from .common import ApiBase


class EndpointAutoConfigAsync(ApiBase):
    async def update(
        self, app_id: str, endpoint_id: str, subscribe_in: SubscribeIn
    ) -> EndpointOut:
        """Update an auto-config endpoint by providing endpoint details."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=subscribe_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())


class EndpointAutoConfig(ApiBase):
    def update(
        self, app_id: str, endpoint_id: str, subscribe_in: SubscribeIn
    ) -> EndpointOut:
        """Update an auto-config endpoint by providing endpoint details."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=subscribe_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())
