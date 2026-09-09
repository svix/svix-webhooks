# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    DestinationIn,
    DestinationOut,
    DestinationPatch,
    ListResponseDestinationOut,
)
from .common import ApiBaseAsync, ApiBaseSync, BaseOptions, serialize_params
from .destination_transformation import (
    DestinationTransformation,
    DestinationTransformationAsync,
)


@dataclass
class DestinationListOptions(BaseOptions):
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
class DestinationCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class DestinationAsync(ApiBaseAsync):
    @property
    def transformation(self) -> DestinationTransformationAsync:
        return DestinationTransformationAsync(self._client, self._httpx_client)

    async def list(
        self, app_id: str, options: DestinationListOptions = (DestinationListOptions())
    ) -> ListResponseDestinationOut:
        """List of all the application's destinations."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/destination",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseDestinationOut.model_validate(response.json())

    async def create(
        self,
        app_id: str,
        destination_in: DestinationIn,
        options: DestinationCreateOptions = (DestinationCreateOptions()),
    ) -> DestinationOut:
        """Creates a new destination."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/destination",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())

    async def get(self, app_id: str, destination_id: str) -> DestinationOut:
        """Get a destination by id or uid."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )
        return DestinationOut.model_validate(response.json())

    async def upsert(
        self, app_id: str, destination_id: str, destination_in: DestinationIn
    ) -> DestinationOut:
        """Create or update a destination."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())

    async def delete(self, app_id: str, destination_id: str) -> None:
        """Delete a destination."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )

    async def patch(
        self, app_id: str, destination_id: str, destination_patch: DestinationPatch
    ) -> DestinationOut:
        """Partially update a destination."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return DestinationOut.model_validate(response.json())


class Destination(ApiBaseSync):
    @property
    def transformation(self) -> DestinationTransformation:
        return DestinationTransformation(self._client, self._httpx_client)

    def list(
        self, app_id: str, options: DestinationListOptions = (DestinationListOptions())
    ) -> ListResponseDestinationOut:
        """List of all the application's destinations."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/destination",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseDestinationOut.model_validate(response.json())

    def create(
        self,
        app_id: str,
        destination_in: DestinationIn,
        options: DestinationCreateOptions = (DestinationCreateOptions()),
    ) -> DestinationOut:
        """Creates a new destination."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/destination",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())

    def get(self, app_id: str, destination_id: str) -> DestinationOut:
        """Get a destination by id or uid."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )
        return DestinationOut.model_validate(response.json())

    def upsert(
        self, app_id: str, destination_id: str, destination_in: DestinationIn
    ) -> DestinationOut:
        """Create or update a destination."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DestinationOut.model_validate(response.json())

    def delete(self, app_id: str, destination_id: str) -> None:
        """Delete a destination."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
        )

    def patch(
        self, app_id: str, destination_id: str, destination_patch: DestinationPatch
    ) -> DestinationOut:
        """Partially update a destination."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/destination/{destination_id}",
            path_params={
                "app_id": app_id,
                "destination_id": destination_id,
            },
            json_body=destination_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return DestinationOut.model_validate(response.json())
