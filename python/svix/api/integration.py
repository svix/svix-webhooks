from dataclasses import dataclass


from ..internal.openapi_client.api.integration import (
    v1_integration_create,
    v1_integration_delete,
    v1_integration_get,
    v1_integration_get_key,
    v1_integration_list,
    v1_integration_rotate_key,
    v1_integration_update,
)
from ..internal.openapi_client.models.integration_in import IntegrationIn
from ..internal.openapi_client.models.integration_key_out import IntegrationKeyOut
from ..internal.openapi_client.models.integration_out import IntegrationOut
from ..internal.openapi_client.models.integration_update import IntegrationUpdate
from ..internal.openapi_client.models.list_response_integration_out import (
    ListResponseIntegrationOut,
)
from .common import ListOptions, PostOptions, ApiBase


@dataclass
class IntegrationListOptions(ListOptions):
    pass


class IntegrationAsync(ApiBase):
    async def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return await v1_integration_list.request_asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(
        self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()
    ) -> IntegrationOut:
        return await v1_integration_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return await v1_integration_get.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def update(
        self, app_id: str, integ_id: str, integ_update: IntegrationUpdate
    ) -> IntegrationOut:
        return await v1_integration_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    async def delete(self, app_id: str, integ_id: str) -> None:
        return await v1_integration_delete.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return await v1_integration_get_key.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def rotate_key(
        self, app_id: str, integ_id: str, options: PostOptions = PostOptions()
    ) -> IntegrationKeyOut:
        return await v1_integration_rotate_key.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )


class Integration(ApiBase):
    def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return v1_integration_list.request_sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(
        self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()
    ) -> IntegrationOut:
        return v1_integration_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return v1_integration_get.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def update(
        self, app_id: str, integ_id: str, integ_update: IntegrationUpdate
    ) -> IntegrationOut:
        return v1_integration_update.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    def delete(self, app_id: str, integ_id: str) -> None:
        return v1_integration_delete.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return v1_integration_get_key.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def rotate_key(
        self, app_id: str, integ_id: str, options: PostOptions = PostOptions()
    ) -> IntegrationKeyOut:
        return v1_integration_rotate_key.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )
