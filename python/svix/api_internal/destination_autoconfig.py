# This file is @generated
from ..models import DestinationIn, DestinationOut
from .common import ApiBaseAsync, ApiBaseSync


class DestinationAutoconfigAsync(ApiBaseAsync):
    async def subscribe(
        self, app_id: str, autoconfig_id: str, destination_in: DestinationIn
    ) -> DestinationOut:
        """Create or update the destination for an AutoConfig subscription."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())


class DestinationAutoconfig(ApiBaseSync):
    def subscribe(
        self, app_id: str, autoconfig_id: str, destination_in: DestinationIn
    ) -> DestinationOut:
        """Create or update the destination for an AutoConfig subscription."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())
