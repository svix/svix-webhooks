import typing as t
from dataclasses import dataclass

from .common import ApiBase, BaseOptions
from ..internal.openapi_client import models


from ..internal.openapi_client.api.event_type import (
    v1_event_type_list,
    v1_event_type_create,
    v1_event_type_import_openapi,
    v1_event_type_get,
    v1_event_type_update,
    v1_event_type_delete,
    v1_event_type_patch,
)

from ..internal.openapi_client.models.list_response_event_type_out import (
    ListResponseEventTypeOut,
)
from ..internal.openapi_client.models.event_type_in import EventTypeIn
from ..internal.openapi_client.models.event_type_out import EventTypeOut
from ..internal.openapi_client.models.event_type_import_open_api_in import (
    EventTypeImportOpenApiIn,
)
from ..internal.openapi_client.models.event_type_import_open_api_out import (
    EventTypeImportOpenApiOut,
)
from ..internal.openapi_client.models.event_type_update import EventTypeUpdate
from ..internal.openapi_client.models.event_type_patch import EventTypePatch


@dataclass
class EventTypeListOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # The sorting order of the returned items
    order: t.Optional[models.Ordering] = None
    # When `true` archived (deleted but not expunged) items are included in the response.
    include_archived: t.Optional[bool] = None
    # When `true` the full item (including the schema) is included in the response.
    with_content: t.Optional[bool] = None


@dataclass
class EventTypeCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EventTypeImportOpenapiOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EventTypeDeleteOptions(BaseOptions):
    # By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
    expunge: t.Optional[bool] = None


class EventTypeAsync(ApiBase):
    async def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        """Return the list of event types."""
        return await v1_event_type_list.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def create(
        self,
        event_type_in: EventTypeIn,
        options: EventTypeCreateOptions = EventTypeCreateOptions(),
    ) -> EventTypeOut:
        """Create new or unarchive existing event type.

        Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        Endpoints filtering on the event type before archival will continue to filter on it.
        This operation does not preserve the description and schemas."""
        return await v1_event_type_create.request_asyncio(
            client=self._client, json_body=event_type_in, **options.to_dict()
        )

    async def import_openapi(
        self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: EventTypeImportOpenapiOptions = EventTypeImportOpenapiOptions(),
    ) -> EventTypeImportOpenApiOut:
        """Given an OpenAPI spec, create new or update existing event types.
        If an existing `archived` event type is updated, it will be unarchived.

        The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        top-level."""
        return await v1_event_type_import_openapi.request_asyncio(
            client=self._client,
            json_body=event_type_import_open_api_in,
            **options.to_dict(),
        )

    async def get(self, event_type_name: str) -> EventTypeOut:
        """Get an event type."""
        return await v1_event_type_get.request_asyncio(
            client=self._client, event_type_name=event_type_name
        )

    async def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        """Update an event type."""
        return await v1_event_type_update.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    async def delete(
        self,
        event_type_name: str,
        options: EventTypeDeleteOptions = EventTypeDeleteOptions(),
    ) -> None:
        """Archive an event type.

        Endpoints already configured to filter on an event type will continue to do so after archival.
        However, new messages can not be sent with it and endpoints can not filter on it.
        An event type can be unarchived with the
        [create operation](#operation/create_event_type_api_v1_event_type__post)."""
        return await v1_event_type_delete.request_asyncio(
            client=self._client, event_type_name=event_type_name, **options.to_dict()
        )

    async def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        """Partially update an event type."""
        return await v1_event_type_patch.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )


class EventType(ApiBase):
    def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        """Return the list of event types."""
        return v1_event_type_list.request_sync(client=self._client, **options.to_dict())

    def create(
        self,
        event_type_in: EventTypeIn,
        options: EventTypeCreateOptions = EventTypeCreateOptions(),
    ) -> EventTypeOut:
        """Create new or unarchive existing event type.

        Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        Endpoints filtering on the event type before archival will continue to filter on it.
        This operation does not preserve the description and schemas."""
        return v1_event_type_create.request_sync(
            client=self._client, json_body=event_type_in, **options.to_dict()
        )

    def import_openapi(
        self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: EventTypeImportOpenapiOptions = EventTypeImportOpenapiOptions(),
    ) -> EventTypeImportOpenApiOut:
        """Given an OpenAPI spec, create new or update existing event types.
        If an existing `archived` event type is updated, it will be unarchived.

        The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        top-level."""
        return v1_event_type_import_openapi.request_sync(
            client=self._client,
            json_body=event_type_import_open_api_in,
            **options.to_dict(),
        )

    def get(self, event_type_name: str) -> EventTypeOut:
        """Get an event type."""
        return v1_event_type_get.request_sync(
            client=self._client, event_type_name=event_type_name
        )

    def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        """Update an event type."""
        return v1_event_type_update.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    def delete(
        self,
        event_type_name: str,
        options: EventTypeDeleteOptions = EventTypeDeleteOptions(),
    ) -> None:
        """Archive an event type.

        Endpoints already configured to filter on an event type will continue to do so after archival.
        However, new messages can not be sent with it and endpoints can not filter on it.
        An event type can be unarchived with the
        [create operation](#operation/create_event_type_api_v1_event_type__post)."""
        return v1_event_type_delete.request_sync(
            client=self._client, event_type_name=event_type_name, **options.to_dict()
        )

    def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        """Partially update an event type."""
        return v1_event_type_patch.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )
