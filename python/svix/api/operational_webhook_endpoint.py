# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    ListResponseOperationalWebhookEndpointOut,
    OperationalWebhookEndpointHeadersIn,
    OperationalWebhookEndpointHeadersOut,
    OperationalWebhookEndpointIn,
    OperationalWebhookEndpointOut,
    OperationalWebhookEndpointSecretIn,
    OperationalWebhookEndpointSecretOut,
    OperationalWebhookEndpointUpdate,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class OperationalWebhookEndpointListOptions(BaseOptions):
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
class OperationalWebhookEndpointCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class OperationalWebhookEndpointRotateSecretOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class OperationalWebhookEndpointAsync(ApiBase):
    async def list(
        self,
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions(),
    ) -> ListResponseOperationalWebhookEndpointOut:
        """List operational webhook endpoints."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/operational-webhook/endpoint",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseOperationalWebhookEndpointOut.model_validate(response.json())

    async def create(
        self,
        operational_webhook_endpoint_in: OperationalWebhookEndpointIn,
        options: OperationalWebhookEndpointCreateOptions = OperationalWebhookEndpointCreateOptions(),
    ) -> OperationalWebhookEndpointOut:
        """Create an operational webhook endpoint."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/operational-webhook/endpoint",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=operational_webhook_endpoint_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    async def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        """Get an operational webhook endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    async def update(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_update: OperationalWebhookEndpointUpdate,
    ) -> OperationalWebhookEndpointOut:
        """Update an operational webhook endpoint."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
            json_body=operational_webhook_endpoint_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    async def delete(self, endpoint_id: str) -> None:
        """Delete an operational webhook endpoint."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )

    async def get_headers(
        self, endpoint_id: str
    ) -> OperationalWebhookEndpointHeadersOut:
        """Get the additional headers to be sent with the operational webhook."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointHeadersOut.model_validate(response.json())

    async def update_headers(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_headers_in: OperationalWebhookEndpointHeadersIn,
    ) -> None:
        """Set the additional headers to be sent with the operational webhook."""
        await self._request_asyncio(
            method="put",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
            path_params={
                "endpoint_id": endpoint_id,
            },
            json_body=operational_webhook_endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    async def get_secret(self, endpoint_id: str) -> OperationalWebhookEndpointSecretOut:
        """Get an operational webhook endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointSecretOut.model_validate(response.json())

    async def rotate_secret(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_secret_in: OperationalWebhookEndpointSecretIn,
        options: OperationalWebhookEndpointRotateSecretOptions = OperationalWebhookEndpointRotateSecretOptions(),
    ) -> None:
        """Rotates an operational webhook endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=operational_webhook_endpoint_secret_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class OperationalWebhookEndpoint(ApiBase):
    def list(
        self,
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions(),
    ) -> ListResponseOperationalWebhookEndpointOut:
        """List operational webhook endpoints."""
        response = self._request_sync(
            method="get",
            path="/api/v1/operational-webhook/endpoint",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseOperationalWebhookEndpointOut.model_validate(response.json())

    def create(
        self,
        operational_webhook_endpoint_in: OperationalWebhookEndpointIn,
        options: OperationalWebhookEndpointCreateOptions = OperationalWebhookEndpointCreateOptions(),
    ) -> OperationalWebhookEndpointOut:
        """Create an operational webhook endpoint."""
        response = self._request_sync(
            method="post",
            path="/api/v1/operational-webhook/endpoint",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=operational_webhook_endpoint_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        """Get an operational webhook endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    def update(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_update: OperationalWebhookEndpointUpdate,
    ) -> OperationalWebhookEndpointOut:
        """Update an operational webhook endpoint."""
        response = self._request_sync(
            method="put",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
            json_body=operational_webhook_endpoint_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return OperationalWebhookEndpointOut.model_validate(response.json())

    def delete(self, endpoint_id: str) -> None:
        """Delete an operational webhook endpoint."""
        self._request_sync(
            method="delete",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )

    def get_headers(self, endpoint_id: str) -> OperationalWebhookEndpointHeadersOut:
        """Get the additional headers to be sent with the operational webhook."""
        response = self._request_sync(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointHeadersOut.model_validate(response.json())

    def update_headers(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_headers_in: OperationalWebhookEndpointHeadersIn,
    ) -> None:
        """Set the additional headers to be sent with the operational webhook."""
        self._request_sync(
            method="put",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
            path_params={
                "endpoint_id": endpoint_id,
            },
            json_body=operational_webhook_endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    def get_secret(self, endpoint_id: str) -> OperationalWebhookEndpointSecretOut:
        """Get an operational webhook endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = self._request_sync(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointSecretOut.model_validate(response.json())

    def rotate_secret(
        self,
        endpoint_id: str,
        operational_webhook_endpoint_secret_in: OperationalWebhookEndpointSecretIn,
        options: OperationalWebhookEndpointRotateSecretOptions = OperationalWebhookEndpointRotateSecretOptions(),
    ) -> None:
        """Rotates an operational webhook endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
        self._request_sync(
            method="post",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=operational_webhook_endpoint_secret_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
