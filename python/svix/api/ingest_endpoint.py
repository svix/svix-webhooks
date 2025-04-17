# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    IngestEndpointHeadersIn,
    IngestEndpointHeadersOut,
    IngestEndpointIn,
    IngestEndpointOut,
    IngestEndpointSecretIn,
    IngestEndpointSecretOut,
    IngestEndpointUpdate,
    ListResponseIngestEndpointOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class IngestEndpointListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
            }
        )


@dataclass
class IngestEndpointCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class IngestEndpointRotateSecretOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class IngestEndpointAsync(ApiBase):
    async def list(
        self,
        source_id: str,
        options: IngestEndpointListOptions = IngestEndpointListOptions(),
    ) -> ListResponseIngestEndpointOut:
        """List ingest endpoints."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIngestEndpointOut.model_validate(response.json())

    async def create(
        self,
        source_id: str,
        ingest_endpoint_in: IngestEndpointIn,
        options: IngestEndpointCreateOptions = IngestEndpointCreateOptions(),
    ) -> IngestEndpointOut:
        """Create an ingest endpoint."""
        response = await self._request_asyncio(
            method="post",
            path="/ingest/api/v1/source/{source_id}/endpoint",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_endpoint_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestEndpointOut.model_validate(response.json())

    async def get(self, source_id: str, endpoint_id: str) -> IngestEndpointOut:
        """Get an ingest endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointOut.model_validate(response.json())

    async def update(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_update: IngestEndpointUpdate,
    ) -> IngestEndpointOut:
        """Update an ingest endpoint."""
        response = await self._request_asyncio(
            method="put",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestEndpointOut.model_validate(response.json())

    async def delete(self, source_id: str, endpoint_id: str) -> None:
        """Delete an ingest endpoint."""
        await self._request_asyncio(
            method="delete",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )

    async def get_headers(
        self, source_id: str, endpoint_id: str
    ) -> IngestEndpointHeadersOut:
        """Get the additional headers to be sent with the ingest."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointHeadersOut.model_validate(response.json())

    async def update_headers(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_headers_in: IngestEndpointHeadersIn,
    ) -> None:
        """Set the additional headers to be sent to the endpoint."""
        await self._request_asyncio(
            method="put",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    async def get_secret(
        self, source_id: str, endpoint_id: str
    ) -> IngestEndpointSecretOut:
        """Get an ingest endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointSecretOut.model_validate(response.json())

    async def rotate_secret(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_secret_in: IngestEndpointSecretIn,
        options: IngestEndpointRotateSecretOptions = IngestEndpointRotateSecretOptions(),
    ) -> None:
        """Rotates an ingest endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
        await self._request_asyncio(
            method="post",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_endpoint_secret_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class IngestEndpoint(ApiBase):
    def list(
        self,
        source_id: str,
        options: IngestEndpointListOptions = IngestEndpointListOptions(),
    ) -> ListResponseIngestEndpointOut:
        """List ingest endpoints."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIngestEndpointOut.model_validate(response.json())

    def create(
        self,
        source_id: str,
        ingest_endpoint_in: IngestEndpointIn,
        options: IngestEndpointCreateOptions = IngestEndpointCreateOptions(),
    ) -> IngestEndpointOut:
        """Create an ingest endpoint."""
        response = self._request_sync(
            method="post",
            path="/ingest/api/v1/source/{source_id}/endpoint",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_endpoint_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestEndpointOut.model_validate(response.json())

    def get(self, source_id: str, endpoint_id: str) -> IngestEndpointOut:
        """Get an ingest endpoint."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointOut.model_validate(response.json())

    def update(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_update: IngestEndpointUpdate,
    ) -> IngestEndpointOut:
        """Update an ingest endpoint."""
        response = self._request_sync(
            method="put",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestEndpointOut.model_validate(response.json())

    def delete(self, source_id: str, endpoint_id: str) -> None:
        """Delete an ingest endpoint."""
        self._request_sync(
            method="delete",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )

    def get_headers(self, source_id: str, endpoint_id: str) -> IngestEndpointHeadersOut:
        """Get the additional headers to be sent with the ingest."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointHeadersOut.model_validate(response.json())

    def update_headers(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_headers_in: IngestEndpointHeadersIn,
    ) -> None:
        """Set the additional headers to be sent to the endpoint."""
        self._request_sync(
            method="put",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            json_body=ingest_endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    def get_secret(self, source_id: str, endpoint_id: str) -> IngestEndpointSecretOut:
        """Get an ingest endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
        )
        return IngestEndpointSecretOut.model_validate(response.json())

    def rotate_secret(
        self,
        source_id: str,
        endpoint_id: str,
        ingest_endpoint_secret_in: IngestEndpointSecretIn,
        options: IngestEndpointRotateSecretOptions = IngestEndpointRotateSecretOptions(),
    ) -> None:
        """Rotates an ingest endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
        self._request_sync(
            method="post",
            path="/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "source_id": source_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_endpoint_secret_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
