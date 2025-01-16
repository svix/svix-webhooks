import typing as t
from dataclasses import dataclass

from .common import ApiBase, BaseOptions
from ..internal.openapi_client import models


from ..internal.openapi_client.api.application import (
    v1_application_list,
    v1_application_create,
    v1_application_get,
    v1_application_update,
    v1_application_delete,
    v1_application_patch,
)

from ..internal.openapi_client.models.list_response_application_out import (
    ListResponseApplicationOut,
)
from ..internal.openapi_client.models.application_in import ApplicationIn
from ..internal.openapi_client.models.application_out import ApplicationOut
from ..internal.openapi_client.models.application_patch import ApplicationPatch


@dataclass
class ApplicationListOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # The sorting order of the returned items
    order: t.Optional[models.Ordering] = None


@dataclass
class ApplicationCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class ApplicationGetOrCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class ApplicationAsync(ApiBase):
    async def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        """List of all the organization's applications."""
        return await v1_application_list.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        """Create a new application."""
        return await v1_application_create.request_asyncio(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    async def get(self, app_id: str) -> ApplicationOut:
        """Get an application."""
        return await v1_application_get.request_asyncio(
            client=self._client, app_id=app_id
        )

    async def get_or_create(
        self,
        application_in: ApplicationIn,
        options: ApplicationGetOrCreateOptions = ApplicationGetOrCreateOptions(),
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
        """Update an application."""
        return await v1_application_update.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_in
        )

    async def delete(self, app_id: str) -> None:
        """Delete an application."""
        return await v1_application_delete.request_asyncio(
            client=self._client, app_id=app_id
        )

    async def patch(
        self, app_id: str, application_patch: ApplicationPatch
    ) -> ApplicationOut:
        """Partially update an application."""
        return await v1_application_patch.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_patch
        )


class Application(ApiBase):
    def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        """List of all the organization's applications."""
        return v1_application_list.request_sync(
            client=self._client, **options.to_dict()
        )

    def create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        """Create a new application."""
        return v1_application_create.request_sync(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    def get(self, app_id: str) -> ApplicationOut:
        """Get an application."""
        return v1_application_get.request_sync(client=self._client, app_id=app_id)

    def get_or_create(
        self,
        application_in: ApplicationIn,
        options: ApplicationGetOrCreateOptions = ApplicationGetOrCreateOptions(),
    ) -> ApplicationOut:
        return v1_application_create.request_sync(
            client=self._client,
            json_body=application_in,
            get_if_exists=True,
            **options.to_dict(),
        )

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        """Update an application."""
        return v1_application_update.request_sync(
            client=self._client, app_id=app_id, json_body=application_in
        )

    def delete(self, app_id: str) -> None:
        """Delete an application."""
        return v1_application_delete.request_sync(client=self._client, app_id=app_id)

    def patch(self, app_id: str, application_patch: ApplicationPatch) -> ApplicationOut:
        """Partially update an application."""
        return v1_application_patch.request_sync(
            client=self._client, app_id=app_id, json_body=application_patch
        )


__all__ = ["ApplicationIn", "ApplicationOut", "ApplicationPatch"]
