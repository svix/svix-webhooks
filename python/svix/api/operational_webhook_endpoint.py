# This file is @generated
import typing as t
from dataclasses import dataclass

from ..internal.openapi_client import models
from ..internal.openapi_client.models.list_response_operational_webhook_endpoint_out import (
    ListResponseOperationalWebhookEndpointOut,
)
from ..internal.openapi_client.models.operational_webhook_endpoint_in import (
    OperationalWebhookEndpointIn,
)
from ..internal.openapi_client.models.operational_webhook_endpoint_out import (
    OperationalWebhookEndpointOut,
)
from ..internal.openapi_client.models.operational_webhook_endpoint_secret_in import (
    OperationalWebhookEndpointSecretIn,
)
from ..internal.openapi_client.models.operational_webhook_endpoint_secret_out import (
    OperationalWebhookEndpointSecretOut,
)
from ..internal.openapi_client.models.operational_webhook_endpoint_update import (
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
        return ListResponseOperationalWebhookEndpointOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_in.to_dict(),
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

    async def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        """Get an operational webhook endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_update.to_dict(),
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

    async def delete(self, endpoint_id: str) -> None:
        """Delete an operational webhook endpoint."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
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
        return OperationalWebhookEndpointSecretOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_secret_in.to_dict(),
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
        return ListResponseOperationalWebhookEndpointOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_in.to_dict(),
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

    def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        """Get an operational webhook endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_update.to_dict(),
        )
        return OperationalWebhookEndpointOut.from_dict(response.json())

    def delete(self, endpoint_id: str) -> None:
        """Delete an operational webhook endpoint."""
        self._request_sync(
            method="delete",
            path="/api/v1/operational-webhook/endpoint/{endpoint_id}",
            path_params={
                "endpoint_id": endpoint_id,
            },
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
        return OperationalWebhookEndpointSecretOut.from_dict(response.json())

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
            json_body=operational_webhook_endpoint_secret_in.to_dict(),
        )
