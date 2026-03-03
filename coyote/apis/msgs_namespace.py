# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgNamespaceCreateIn,
    MsgNamespaceCreateOut,
    MsgNamespaceGetIn,
    MsgNamespaceGetOut,
)

from ..models.msg_namespace_create_in import _MsgNamespaceCreateIn
from ..models.msg_namespace_get_in import _MsgNamespaceGetIn


class MsgsNamespaceAsync(ApiBase):
    async def create(
        self,
        name: str,
        msg_namespace_create_in: MsgNamespaceCreateIn,
    ) -> MsgNamespaceCreateOut:
        """Creates or updates a msgs namespace with the given name."""
        body = _MsgNamespaceCreateIn(
            name=name,
            retention=msg_namespace_create_in.retention,
            storage_type=msg_namespace_create_in.storage_type,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=body,
            response_type=MsgNamespaceCreateOut,
        )

    async def get(
        self,
        name: str,
        msg_namespace_get_in: MsgNamespaceGetIn,
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = _MsgNamespaceGetIn(
            name=name,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=body,
            response_type=MsgNamespaceGetOut,
        )


class MsgsNamespace(ApiBase):
    def create(
        self,
        name: str,
        msg_namespace_create_in: MsgNamespaceCreateIn,
    ) -> MsgNamespaceCreateOut:
        """Creates or updates a msgs namespace with the given name."""
        body = _MsgNamespaceCreateIn(
            name=name,
            retention=msg_namespace_create_in.retention,
            storage_type=msg_namespace_create_in.storage_type,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=body,
            response_type=MsgNamespaceCreateOut,
        )

    def get(
        self,
        name: str,
        msg_namespace_get_in: MsgNamespaceGetIn,
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = _MsgNamespaceGetIn(
            name=name,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=body,
            response_type=MsgNamespaceGetOut,
        )
