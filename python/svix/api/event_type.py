# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    EventTypeImportOpenApiIn,
    EventTypeImportOpenApiOut,
    EventTypeIn,
    EventTypeOut,
    EventTypePatch,
    EventTypeUpdate,
    ListResponseEventTypeOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class EventTypeListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""
    include_archived: t.Optional[bool] = None
    """When `true` archived (deleted but not expunged) items are included in the response."""
    with_content: t.Optional[bool] = None
    """When `true` the full item (including the schema) is included in the response."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
                "include_archived": self.include_archived,
                "with_content": self.with_content,
            }
        )


@dataclass
class EventTypeCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EventTypeImportOpenapiOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EventTypeDeleteOptions(BaseOptions):
    expunge: t.Optional[bool] = None
    """By default event types are archived when "deleted". Passing this to `true` deletes them entirely."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "expunge": self.expunge,
            }
        )


class EventTypeAsync(ApiBase):
    async def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        """Return the list of event types."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEventTypeOut.model_validate(response.json())

    async def create(
        self,
        event_type_in: EventTypeIn,
        options: EventTypeCreateOptions = EventTypeCreateOptions(),
    ) -> EventTypeOut:
        """Create new or unarchive existing event type.

        Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        Endpoints filtering on the event type before archival will continue to filter on it.
        This operation does not preserve the description and schemas."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_type_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EventTypeOut.model_validate(response.json())

    async def import_openapi(
        self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: EventTypeImportOpenapiOptions = EventTypeImportOpenapiOptions(),
    ) -> EventTypeImportOpenApiOut:
        """Given an OpenAPI spec, create new or update existing event types.
        If an existing `archived` event type is updated, it will be unarchived.

        The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        top-level."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/event-type/import/openapi",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_type_import_open_api_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeImportOpenApiOut.model_validate(response.json())

    async def get(self, event_type_name: str) -> EventTypeOut:
        """Get an event type."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
        )
        return EventTypeOut.model_validate(response.json())

    async def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        """Update an event type."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            json_body=event_type_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeOut.model_validate(response.json())

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
        await self._request_asyncio(
            method="delete",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )

    async def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        """Partially update an event type."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            json_body=event_type_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeOut.model_validate(response.json())


class EventType(ApiBase):
    def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        """Return the list of event types."""
        response = self._request_sync(
            method="get",
            path="/api/v1/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEventTypeOut.model_validate(response.json())

    def create(
        self,
        event_type_in: EventTypeIn,
        options: EventTypeCreateOptions = EventTypeCreateOptions(),
    ) -> EventTypeOut:
        """Create new or unarchive existing event type.

        Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
        Endpoints filtering on the event type before archival will continue to filter on it.
        This operation does not preserve the description and schemas."""
        response = self._request_sync(
            method="post",
            path="/api/v1/event-type",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_type_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EventTypeOut.model_validate(response.json())

    def import_openapi(
        self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: EventTypeImportOpenapiOptions = EventTypeImportOpenapiOptions(),
    ) -> EventTypeImportOpenApiOut:
        """Given an OpenAPI spec, create new or update existing event types.
        If an existing `archived` event type is updated, it will be unarchived.

        The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
        top-level."""
        response = self._request_sync(
            method="post",
            path="/api/v1/event-type/import/openapi",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_type_import_open_api_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeImportOpenApiOut.model_validate(response.json())

    def get(self, event_type_name: str) -> EventTypeOut:
        """Get an event type."""
        response = self._request_sync(
            method="get",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
        )
        return EventTypeOut.model_validate(response.json())

    def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        """Update an event type."""
        response = self._request_sync(
            method="put",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            json_body=event_type_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeOut.model_validate(response.json())

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
        self._request_sync(
            method="delete",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )

    def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        """Partially update an event type."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/event-type/{event_type_name}",
            path_params={
                "event_type_name": event_type_name,
            },
            json_body=event_type_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EventTypeOut.model_validate(response.json())
