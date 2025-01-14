import typing as t
from dataclasses import dataclass


from ..internal.openapi_client.api.application import (
    v1_application_create,
    v1_application_delete,
    v1_application_get,
    v1_application_list,
    v1_application_patch,
    v1_application_update,
)
from ..internal.openapi_client.models.application_in import ApplicationIn
from ..internal.openapi_client.models.application_out import ApplicationOut
from ..internal.openapi_client.models.application_patch import ApplicationPatch
from ..internal.openapi_client.models.list_response_application_out import (
    ListResponseApplicationOut,
)
from ..internal.openapi_client.models.ordering import Ordering

from .common import PostOptions, ApiBase, ListOptions


@dataclass
class ApplicationListOptions(ListOptions):
    order: t.Optional[Ordering] = None


class ApplicationAsync(ApiBase):
    async def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        return await v1_application_list.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return await v1_application_create.request_asyncio(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    async def get(self, app_id: str) -> ApplicationOut:
        return await v1_application_get.request_asyncio(
            client=self._client, app_id=app_id
        )

    async def get_or_create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return await v1_application_create.request_asyncio(
            client=self._client,
            json_body=application_in,
            get_if_exists=True,
            **options.to_dict(),
        )

    async def update(
        self, app_id: str, application_in: ApplicationIn
    ) -> ApplicationOut:
        return await v1_application_update.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_in
        )

    async def patch(
        self, app_id: str, application_patch: ApplicationPatch
    ) -> ApplicationOut:
        return await v1_application_patch.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_patch
        )

    async def delete(self, app_id: str) -> None:
        return await v1_application_delete.request_asyncio(
            client=self._client, app_id=app_id
        )


class Application(ApiBase):
    def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        return v1_application_list.request_sync(
            client=self._client, **options.to_dict()
        )

    def create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return v1_application_create.request_sync(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    def get(self, app_id: str) -> ApplicationOut:
        return v1_application_get.request_sync(client=self._client, app_id=app_id)

    def get_or_create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return v1_application_create.request_sync(
            client=self._client,
            json_body=application_in,
            get_if_exists=True,
            **options.to_dict(),
        )

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        return v1_application_update.request_sync(
            client=self._client, app_id=app_id, json_body=application_in
        )

    def patch(self, app_id: str, application_patch: ApplicationPatch) -> ApplicationOut:
        return v1_application_patch.request_sync(
            client=self._client, app_id=app_id, json_body=application_patch
        )

    def delete(self, app_id: str) -> None:
        return v1_application_delete.request_sync(client=self._client, app_id=app_id)


__all__ = ["ApplicationIn", "ApplicationOut", "ApplicationPatch"]
