# This file is @generated
from ..models import EndpointIn, EndpointOut
from .common import ApiBaseAsync, ApiBaseSync


class EndpointAutoconfigAsync(ApiBaseAsync):
    async def subscribe(
        self, app_id: str, autoconfig_id: str, endpoint_in: EndpointIn
    ) -> EndpointOut:
        """Create or update the HTTP endpoint for an AutoConfig subscription."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            json_body=endpoint_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())


class EndpointAutoconfig(ApiBaseSync):
    def subscribe(
        self, app_id: str, autoconfig_id: str, endpoint_in: EndpointIn
    ) -> EndpointOut:
        """Create or update the HTTP endpoint for an AutoConfig subscription."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            json_body=endpoint_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())
