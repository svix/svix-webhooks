import typing as t
from dataclasses import dataclass

from .common import ListOptions, PostOptions, ApiBase


from ..internal.openapi_client.api.webhook_endpoint import (
    create_operational_webhook_endpoint,
    delete_operational_webhook_endpoint,
    get_operational_webhook_endpoint,
    get_operational_webhook_endpoint_secret,
    list_operational_webhook_endpoints,
    rotate_operational_webhook_endpoint_secret,
    update_operational_webhook_endpoint,
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
from ..internal.openapi_client.models.list_response_operational_webhook_endpoint_out import (
    ListResponseOperationalWebhookEndpointOut,
)
from ..internal.openapi_client.models.ordering import Ordering


@dataclass
class OperationalWebhookEndpointListOptions(ListOptions):
    order: t.Optional[Ordering] = None


class OperationalWebhookEndpointAsync(ApiBase):
    async def list(
        self,
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions(),
    ) -> ListResponseOperationalWebhookEndpointOut:
        return await list_operational_webhook_endpoints.request_asyncio(
            client=self._client,
            **options.to_dict(),
        )

    async def create(
        self,
        endpoint_in: OperationalWebhookEndpointIn,
        options: PostOptions = PostOptions(),
    ) -> OperationalWebhookEndpointOut:
        return await create_operational_webhook_endpoint.request_asyncio(
            client=self._client,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    async def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        return await get_operational_webhook_endpoint.request_asyncio(
            client=self._client, endpoint_id=endpoint_id
        )

    async def update(
        self, endpoint_id: str, endpoint_update: OperationalWebhookEndpointUpdate
    ) -> OperationalWebhookEndpointOut:
        return await update_operational_webhook_endpoint.request_asyncio(
            client=self._client,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    async def delete(self, endpoint_id: str) -> None:
        return await delete_operational_webhook_endpoint.request_asyncio(
            client=self._client,
            endpoint_id=endpoint_id,
        )

    async def get_secret(self, endpoint_id: str) -> OperationalWebhookEndpointSecretOut:
        return await get_operational_webhook_endpoint_secret.request_asyncio(
            client=self._client,
            endpoint_id=endpoint_id,
        )

    async def rotate_secret(
        self,
        endpoint_id: str,
        endpoint_secret_rotate_in: OperationalWebhookEndpointSecretIn,
        options: PostOptions = PostOptions(),
    ) -> None:
        return await rotate_operational_webhook_endpoint_secret.request_asyncio(
            client=self._client,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )


class OperationalWebhookEndpoint(ApiBase):
    def list(
        self,
        options: OperationalWebhookEndpointListOptions = OperationalWebhookEndpointListOptions(),
    ) -> ListResponseOperationalWebhookEndpointOut:
        return list_operational_webhook_endpoints.request_sync(
            client=self._client,
            **options.to_dict(),
        )

    def create(
        self,
        endpoint_in: OperationalWebhookEndpointIn,
        options: PostOptions = PostOptions(),
    ) -> OperationalWebhookEndpointOut:
        return create_operational_webhook_endpoint.request_sync(
            client=self._client,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    def get(self, endpoint_id: str) -> OperationalWebhookEndpointOut:
        return get_operational_webhook_endpoint.request_sync(
            client=self._client, endpoint_id=endpoint_id
        )

    def update(
        self, endpoint_id: str, endpoint_update: OperationalWebhookEndpointUpdate
    ) -> OperationalWebhookEndpointOut:
        return update_operational_webhook_endpoint.request_sync(
            client=self._client,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    def delete(self, endpoint_id: str) -> None:
        return delete_operational_webhook_endpoint.request_sync(
            client=self._client,
            endpoint_id=endpoint_id,
        )

    def get_secret(self, endpoint_id: str) -> OperationalWebhookEndpointSecretOut:
        return get_operational_webhook_endpoint_secret.request_sync(
            client=self._client,
            endpoint_id=endpoint_id,
        )

    def rotate_secret(
        self,
        endpoint_id: str,
        endpoint_secret_rotate_in: OperationalWebhookEndpointSecretIn,
        options: PostOptions = PostOptions(),
    ) -> None:
        return rotate_operational_webhook_endpoint_secret.request_sync(
            client=self._client,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )
