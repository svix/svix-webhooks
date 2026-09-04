# This file is @generated
from .common import ApiBaseAsync, ApiBaseSync
from .destination_autoconfig import (
    DestinationAutoconfig,
    DestinationAutoconfigAsync,
)


class DestinationAsync(ApiBaseAsync):
    @property
    def autoconfig(self) -> DestinationAutoconfigAsync:
        return DestinationAutoconfigAsync(self._client, self._httpx_client)


class Destination(ApiBaseSync):
    @property
    def autoconfig(self) -> DestinationAutoconfig:
        return DestinationAutoconfig(self._client, self._httpx_client)
