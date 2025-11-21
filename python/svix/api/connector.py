# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    ConnectorIn,
    ConnectorOut,
    ConnectorPatch,
    ConnectorUpdate,
    ListResponseConnectorOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class ConnectorListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""
    product_type: t.Optional[models.ConnectorProduct] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
                "product_type": self.product_type,
            }
        )


@dataclass
class ConnectorCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class ConnectorAsync(ApiBase):
    async def list(
        self, options: ConnectorListOptions = ConnectorListOptions()
    ) -> ListResponseConnectorOut:
        """List all connectors for an application."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/connector",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseConnectorOut.model_validate(response.json())

    async def create(
        self,
        connector_in: ConnectorIn,
        options: ConnectorCreateOptions = ConnectorCreateOptions(),
    ) -> ConnectorOut:
        """Create a new connector."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/connector",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=connector_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ConnectorOut.model_validate(response.json())

    async def get(self, connector_id: str) -> ConnectorOut:
        """Get a connector."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
        )
        return ConnectorOut.model_validate(response.json())

    async def update(
        self, connector_id: str, connector_update: ConnectorUpdate
    ) -> ConnectorOut:
        """Update a connector."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
            json_body=connector_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ConnectorOut.model_validate(response.json())

    async def delete(self, connector_id: str) -> None:
        """Delete a connector."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
        )

    async def patch(
        self, connector_id: str, connector_patch: ConnectorPatch
    ) -> ConnectorOut:
        """Partially update a connector."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
            json_body=connector_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ConnectorOut.model_validate(response.json())


class Connector(ApiBase):
    def list(
        self, options: ConnectorListOptions = ConnectorListOptions()
    ) -> ListResponseConnectorOut:
        """List all connectors for an application."""
        response = self._request_sync(
            method="get",
            path="/api/v1/connector",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseConnectorOut.model_validate(response.json())

    def create(
        self,
        connector_in: ConnectorIn,
        options: ConnectorCreateOptions = ConnectorCreateOptions(),
    ) -> ConnectorOut:
        """Create a new connector."""
        response = self._request_sync(
            method="post",
            path="/api/v1/connector",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=connector_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ConnectorOut.model_validate(response.json())

    def get(self, connector_id: str) -> ConnectorOut:
        """Get a connector."""
        response = self._request_sync(
            method="get",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
        )
        return ConnectorOut.model_validate(response.json())

    def update(
        self, connector_id: str, connector_update: ConnectorUpdate
    ) -> ConnectorOut:
        """Update a connector."""
        response = self._request_sync(
            method="put",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
            json_body=connector_update.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ConnectorOut.model_validate(response.json())

    def delete(self, connector_id: str) -> None:
        """Delete a connector."""
        self._request_sync(
            method="delete",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
        )

    def patch(self, connector_id: str, connector_patch: ConnectorPatch) -> ConnectorOut:
        """Partially update a connector."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/connector/{connector_id}",
            path_params={
                "connector_id": connector_id,
            },
            json_body=connector_patch.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return ConnectorOut.model_validate(response.json())
