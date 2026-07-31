# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import AppPortalAccessOut, IngestSourceConsumerPortalAccessIn
from .common import ApiBaseAsync, ApiBaseSync, BaseOptions, serialize_params


@dataclass
class IngestAuthenticationConsumerPortalAccessOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class IngestAuthenticationAsync(ApiBaseAsync):
    async def consumer_portal_access(
        self,
        source_id: str,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: IngestAuthenticationConsumerPortalAccessOptions = (
            IngestAuthenticationConsumerPortalAccessOptions()
        ),
    ) -> AppPortalAccessOut:
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
        return AppPortalAccessOut.model_validate(response.json())


class IngestAuthentication(ApiBaseSync):
    def consumer_portal_access(
        self,
        source_id: str,
        ingest_source_consumer_portal_access_in: IngestSourceConsumerPortalAccessIn,
        options: IngestAuthenticationConsumerPortalAccessOptions = (
            IngestAuthenticationConsumerPortalAccessOptions()
        ),
    ) -> AppPortalAccessOut:
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
        return AppPortalAccessOut.model_validate(response.json())
