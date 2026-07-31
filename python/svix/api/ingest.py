# This file is @generated
from .common import ApiBaseAsync, ApiBaseSync
from .ingest_authentication import (
    IngestAuthentication,
    IngestAuthenticationAsync,
)
from .ingest_endpoint import (
    IngestEndpoint,
    IngestEndpointAsync,
)
from .ingest_source import (
    IngestSource,
    IngestSourceAsync,
)


class IngestAsync(ApiBaseAsync):
    @property
    def authentication(self) -> IngestAuthenticationAsync:
        return IngestAuthenticationAsync(self._client, self._httpx_client)

    @property
    def endpoint(self) -> IngestEndpointAsync:
        return IngestEndpointAsync(self._client, self._httpx_client)

    @property
    def source(self) -> IngestSourceAsync:
        return IngestSourceAsync(self._client, self._httpx_client)


class Ingest(ApiBaseSync):
    @property
    def authentication(self) -> IngestAuthentication:
        return IngestAuthentication(self._client, self._httpx_client)

    @property
    def endpoint(self) -> IngestEndpoint:
        return IngestEndpoint(self._client, self._httpx_client)

    @property
    def source(self) -> IngestSource:
        return IngestSource(self._client, self._httpx_client)
