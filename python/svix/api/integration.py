import typing as t
from dataclasses import dataclass

from .common import ApiBase, BaseOptions
from ..internal.openapi_client import models


from ..internal.openapi_client.api.integration import (
    v1_integration_list,
    v1_integration_create,
    v1_integration_get,
    v1_integration_update,
    v1_integration_delete,
    v1_integration_get_key,
    v1_integration_rotate_key,
)

from ..internal.openapi_client.models.list_response_integration_out import (
    ListResponseIntegrationOut,
)
from ..internal.openapi_client.models.integration_in import IntegrationIn
from ..internal.openapi_client.models.integration_out import IntegrationOut
from ..internal.openapi_client.models.integration_update import IntegrationUpdate
from ..internal.openapi_client.models.integration_key_out import IntegrationKeyOut


@dataclass
class IntegrationListOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # The sorting order of the returned items
    order: t.Optional[models.Ordering] = None


@dataclass
class IntegrationCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class IntegrationRotateKeyOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class IntegrationAsync(ApiBase):
    async def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        """List the application's integrations."""
        return await v1_integration_list.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def create(
        self,
        app_id: str,
        integration_in: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ) -> IntegrationOut:
        """Create an integration."""
        return await v1_integration_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=integration_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        """Get an integration."""
        return await v1_integration_get.request_asyncio(
            client=self._client, app_id=app_id, integ_id=integ_id
        )

    async def update(
        self, app_id: str, integ_id: str, integration_update: IntegrationUpdate
    ) -> IntegrationOut:
        """Update an integration."""
        return await v1_integration_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integration_update,
        )

    async def delete(self, app_id: str, integ_id: str) -> None:
        """Delete an integration."""
        return await v1_integration_delete.request_asyncio(
            client=self._client, app_id=app_id, integ_id=integ_id
        )

    async def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return await v1_integration_get_key.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def rotate_key(
        self,
        app_id: str,
        integ_id: str,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ) -> IntegrationKeyOut:
        """Rotate the integration's key. The previous key will be immediately revoked."""
        return await v1_integration_rotate_key.request_asyncio(
            client=self._client, app_id=app_id, integ_id=integ_id, **options.to_dict()
        )


class Integration(ApiBase):
    def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        """List the application's integrations."""
        return v1_integration_list.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def create(
        self,
        app_id: str,
        integration_in: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ) -> IntegrationOut:
        """Create an integration."""
        return v1_integration_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=integration_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        """Get an integration."""
        return v1_integration_get.request_sync(
            client=self._client, app_id=app_id, integ_id=integ_id
        )

    def update(
        self, app_id: str, integ_id: str, integration_update: IntegrationUpdate
    ) -> IntegrationOut:
        """Update an integration."""
        return v1_integration_update.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integration_update,
        )

    def delete(self, app_id: str, integ_id: str) -> None:
        """Delete an integration."""
        return v1_integration_delete.request_sync(
            client=self._client, app_id=app_id, integ_id=integ_id
        )

    def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return v1_integration_get_key.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def rotate_key(
        self,
        app_id: str,
        integ_id: str,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ) -> IntegrationKeyOut:
        """Rotate the integration's key. The previous key will be immediately revoked."""
        return v1_integration_rotate_key.request_sync(
            client=self._client, app_id=app_id, integ_id=integ_id, **options.to_dict()
        )
