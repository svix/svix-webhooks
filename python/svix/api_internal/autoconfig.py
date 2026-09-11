# This file is @generated
from ..models import AutoConfigSubscriptionOut
from .autoconfig_destination import (
    AutoconfigDestination,
    AutoconfigDestinationAsync,
)
from .autoconfig_endpoint import (
    AutoconfigEndpoint,
    AutoconfigEndpointAsync,
)
from .common import ApiBaseAsync, ApiBaseSync


class AutoconfigAsync(ApiBaseAsync):
    @property
    def destination(self) -> AutoconfigDestinationAsync:
        return AutoconfigDestinationAsync(self._client, self._httpx_client)

    @property
    def endpoint(self) -> AutoconfigEndpointAsync:
        return AutoconfigEndpointAsync(self._client, self._httpx_client)

    async def get(self, app_id: str, autoconfig_id: str) -> AutoConfigSubscriptionOut:
        """Get an AutoConfig subscription, including the bound endpoint or destination if any."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
        )
        return AutoConfigSubscriptionOut.model_validate(response.json())


class Autoconfig(ApiBaseSync):
    @property
    def destination(self) -> AutoconfigDestination:
        return AutoconfigDestination(self._client, self._httpx_client)

    @property
    def endpoint(self) -> AutoconfigEndpoint:
        return AutoconfigEndpoint(self._client, self._httpx_client)

    def get(self, app_id: str, autoconfig_id: str) -> AutoConfigSubscriptionOut:
        """Get an AutoConfig subscription, including the bound endpoint or destination if any."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
        )
        return AutoConfigSubscriptionOut.model_validate(response.json())
