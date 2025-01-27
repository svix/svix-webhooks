# This file is @generated
import typing as t
from dataclasses import dataclass

from deprecated import deprecated

from .. import models
from ..models import (
    IntegrationIn,
    IntegrationKeyOut,
    IntegrationOut,
    IntegrationUpdate,
    ListResponseIntegrationOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class IntegrationListOptions(BaseOptions):
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
class IntegrationCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class IntegrationRotateKeyOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class IntegrationAsync(ApiBase):
    async def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        """List the application's integrations."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/integration",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIntegrationOut.model_validate(response.json())

    async def create(
        self,
        app_id: str,
        integration_in: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ) -> IntegrationOut:
        """Create an integration."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/integration",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=integration_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return IntegrationOut.model_validate(response.json())

    async def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        """Get an integration."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )
        return IntegrationOut.model_validate(response.json())

    async def update(
        self, app_id: str, integ_id: str, integration_update: IntegrationUpdate
    ) -> IntegrationOut:
        """Update an integration."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
            json_body=integration_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IntegrationOut.model_validate(response.json())

    async def delete(self, app_id: str, integ_id: str) -> None:
        """Delete an integration."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )

    @deprecated
    async def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        """Get an integration's key."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/integration/{integ_id}/key",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )
        return IntegrationKeyOut.model_validate(response.json())

    async def rotate_key(
        self,
        app_id: str,
        integ_id: str,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ) -> IntegrationKeyOut:
        """Rotate the integration's key. The previous key will be immediately revoked."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return IntegrationKeyOut.model_validate(response.json())


class Integration(ApiBase):
    def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        """List the application's integrations."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/integration",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIntegrationOut.model_validate(response.json())

    def create(
        self,
        app_id: str,
        integration_in: IntegrationIn,
        options: IntegrationCreateOptions = IntegrationCreateOptions(),
    ) -> IntegrationOut:
        """Create an integration."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/integration",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=integration_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return IntegrationOut.model_validate(response.json())

    def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        """Get an integration."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )
        return IntegrationOut.model_validate(response.json())

    def update(
        self, app_id: str, integ_id: str, integration_update: IntegrationUpdate
    ) -> IntegrationOut:
        """Update an integration."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
            json_body=integration_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IntegrationOut.model_validate(response.json())

    def delete(self, app_id: str, integ_id: str) -> None:
        """Delete an integration."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}/integration/{integ_id}",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )

    @deprecated
    def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        """Get an integration's key."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/integration/{integ_id}/key",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
        )
        return IntegrationKeyOut.model_validate(response.json())

    def rotate_key(
        self,
        app_id: str,
        integ_id: str,
        options: IntegrationRotateKeyOptions = IntegrationRotateKeyOptions(),
    ) -> IntegrationKeyOut:
        """Rotate the integration's key. The previous key will be immediately revoked."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
            path_params={
                "app_id": app_id,
                "integ_id": integ_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return IntegrationKeyOut.model_validate(response.json())
