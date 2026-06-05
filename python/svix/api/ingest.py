# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import DashboardAccessOut, IngestSourceConsumerPortalAccessIn
from .common import ApiBase, BaseOptions, serialize_params
from .ingest_endpoint import (
    IngestEndpoint,
    IngestEndpointAsync,
)
from .ingest_source import (
    IngestSource,
    IngestSourceAsync,
)


@dataclass
class IngestDashboardOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class IngestAsync(ApiBase):
    @property
    def endpoint(self) -> IngestEndpointAsync:
        return IngestEndpointAsync(self._client)

    @property
    def source(self) -> IngestSourceAsync:
        return IngestSourceAsync(self._client)

    async def dashboard(
        self,
        source_id: str,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: IngestDashboardOptions = IngestDashboardOptions(),
    ) -> DashboardAccessOut:
        """Get access to the Ingest Source Consumer Portal."""
        response = await self._request_asyncio(
            method="post",
            path="/ingest/api/v1/source/{source_id}/dashboard",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_source_consumer_portal_access_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return DashboardAccessOut.model_validate(response.json())


class Ingest(ApiBase):
    @property
    def endpoint(self) -> IngestEndpoint:
        return IngestEndpoint(self._client)

    @property
    def source(self) -> IngestSource:
        return IngestSource(self._client)

    def dashboard(
        self,
        source_id: str,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: IngestDashboardOptions = IngestDashboardOptions(),
    ) -> DashboardAccessOut:
        """Get access to the Ingest Source Consumer Portal."""
        response = self._request_sync(
            method="post",
            path="/ingest/api/v1/source/{source_id}/dashboard",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_source_consumer_portal_access_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return DashboardAccessOut.model_validate(response.json())
