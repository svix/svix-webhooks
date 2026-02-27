# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    CreateNamespaceIn,
    CreateNamespaceOut,
    GetNamespaceIn,
    GetNamespaceOut,
)


class MsgsNamespaceAsync(ApiBase):
    async def create(
        self,
        create_namespace_in: CreateNamespaceIn,
    ) -> CreateNamespaceOut:
        """Creates or updates a msgs namespace with the given name."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=create_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CreateNamespaceOut,
        )

    async def get(
        self,
        get_namespace_in: GetNamespaceIn,
    ) -> GetNamespaceOut:
        """Gets a msgs namespace by name."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=GetNamespaceOut,
        )


class MsgsNamespace(ApiBase):
    def create(
        self,
        create_namespace_in: CreateNamespaceIn,
    ) -> CreateNamespaceOut:
        """Creates or updates a msgs namespace with the given name."""
        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=create_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CreateNamespaceOut,
        )

    def get(
        self,
        get_namespace_in: GetNamespaceIn,
    ) -> GetNamespaceOut:
        """Gets a msgs namespace by name."""
        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=GetNamespaceOut,
        )
