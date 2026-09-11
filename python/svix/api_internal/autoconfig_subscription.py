# This file is @generated
from ..models import AutoConfigSubscriptionOut
from .autoconfig_subscription_destination import (
    AutoconfigSubscriptionDestination,
    AutoconfigSubscriptionDestinationAsync,
)
from .autoconfig_subscription_endpoint import (
    AutoconfigSubscriptionEndpoint,
    AutoconfigSubscriptionEndpointAsync,
)
from .common import ApiBaseAsync, ApiBaseSync


class AutoconfigSubscriptionAsync(ApiBaseAsync):
    @property
    def destination(self) -> AutoconfigSubscriptionDestinationAsync:
        return AutoconfigSubscriptionDestinationAsync(self._client, self._httpx_client)

    @property
    def endpoint(self) -> AutoconfigSubscriptionEndpointAsync:
        return AutoconfigSubscriptionEndpointAsync(self._client, self._httpx_client)

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


class AutoconfigSubscription(ApiBaseSync):
    @property
    def destination(self) -> AutoconfigSubscriptionDestination:
        return AutoconfigSubscriptionDestination(self._client, self._httpx_client)

    @property
    def endpoint(self) -> AutoconfigSubscriptionEndpoint:
        return AutoconfigSubscriptionEndpoint(self._client, self._httpx_client)

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
