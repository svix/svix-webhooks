# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    ApplicationIn,
    ApplicationOut,
    ApplicationPatch,
    ListResponseApplicationOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class ApplicationListOptions(BaseOptions):
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
class ApplicationCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class ApplicationAsync(ApiBase):
    async def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        """List of all the organization's applications."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseApplicationOut.model_validate(response.json())

    async def create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        """Create a new application."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    async def get_or_create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app",
            path_params={},
            query_params={"get_if_exists": "true"},
            header_params=options._header_params(),
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    async def get(self, app_id: str) -> ApplicationOut:
        """Get an application."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
        )
        return ApplicationOut.model_validate(response.json())

    async def update(
        self, app_id: str, application_in: ApplicationIn
    ) -> ApplicationOut:
        """Update an application."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    async def delete(self, app_id: str) -> None:
        """Delete an application."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
        )

    async def patch(
        self, app_id: str, application_patch: ApplicationPatch
    ) -> ApplicationOut:
        """Partially update an application."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
            json_body=application_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ApplicationOut.model_validate(response.json())


class Application(ApiBase):
    def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        """List of all the organization's applications."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseApplicationOut.model_validate(response.json())

    def create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        """Create a new application."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    def get_or_create(
        self,
        application_in: ApplicationIn,
        options: ApplicationCreateOptions = ApplicationCreateOptions(),
    ) -> ApplicationOut:
        response = self._request_sync(
            method="post",
            path="/api/v1/app",
            path_params={},
            query_params={"get_if_exists": "true"},
            header_params=options._header_params(),
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    def get(self, app_id: str) -> ApplicationOut:
        """Get an application."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
        )
        return ApplicationOut.model_validate(response.json())

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        """Update an application."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
            json_body=application_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApplicationOut.model_validate(response.json())

    def delete(self, app_id: str) -> None:
        """Delete an application."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
        )

    def patch(self, app_id: str, application_patch: ApplicationPatch) -> ApplicationOut:
        """Partially update an application."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}",
            path_params={
                "app_id": app_id,
            },
            json_body=application_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ApplicationOut.model_validate(response.json())
