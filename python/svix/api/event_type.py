import typing as t
from dataclasses import dataclass


from ..internal.openapi_client.api.event_type import (
    v1_event_type_create,
    v1_event_type_delete,
    v1_event_type_get,
    v1_event_type_import_openapi,
    v1_event_type_list,
    v1_event_type_patch,
    v1_event_type_update,
)
from ..internal.openapi_client.models.event_type_import_open_api_in import (
    EventTypeImportOpenApiIn,
)
from ..internal.openapi_client.models.event_type_import_open_api_out import (
    EventTypeImportOpenApiOut,
)
from ..internal.openapi_client.models.event_type_in import EventTypeIn
from ..internal.openapi_client.models.event_type_out import EventTypeOut
from ..internal.openapi_client.models.event_type_patch import EventTypePatch
from ..internal.openapi_client.models.event_type_update import EventTypeUpdate
from ..internal.openapi_client.models.list_response_event_type_out import (
    ListResponseEventTypeOut,
)
from .common import ListOptions, PostOptions, ApiBase


@dataclass
class EventTypeListOptions(ListOptions):
    with_content: t.Optional[bool] = None
    include_archived: t.Optional[bool] = None


class EventTypeAsync(ApiBase):
    async def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        return await v1_event_type_list.request_asyncio(
            client=self._client,
            **options.to_dict(),
        )

    async def create(
        self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()
    ) -> EventTypeOut:
        return await v1_event_type_create.request_asyncio(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    async def get(self, event_type_name: str) -> EventTypeOut:
        return await v1_event_type_get.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )

    async def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        return await v1_event_type_update.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    async def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        return await v1_event_type_patch.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )

    async def delete(self, event_type_name: str) -> None:
        return await v1_event_type_delete.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )

    async def import_openapi(
        self,
        event_type_import_openapi_in: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ) -> EventTypeImportOpenApiOut:
        return await v1_event_type_import_openapi.request_asyncio(
            client=self._client,
            json_body=event_type_import_openapi_in,
            **options.to_dict(),
        )


class EventType(ApiBase):
    def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        return v1_event_type_list.request_sync(
            client=self._client,
            **options.to_dict(),
        )

    def create(
        self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()
    ) -> EventTypeOut:
        return v1_event_type_create.request_sync(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    def get(self, event_type_name: str) -> EventTypeOut:
        return v1_event_type_get.request_sync(
            client=self._client,
            event_type_name=event_type_name,
        )

    def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        return v1_event_type_update.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        return v1_event_type_patch.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )

    def delete(self, event_type_name: str) -> None:
        return v1_event_type_delete.request_sync(
            client=self._client,
            event_type_name=event_type_name,
        )

    def import_openapi(
        self,
        event_type_import_openapi_in: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ) -> EventTypeImportOpenApiOut:
        return v1_event_type_import_openapi.request_sync(
            client=self._client,
            json_body=event_type_import_openapi_in,
            **options.to_dict(),
        )
