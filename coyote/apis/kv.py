# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    KvDeleteIn,
    KvDeleteOut,
    KvGetIn,
    KvGetNamespaceIn,
    KvGetNamespaceOut,
    KvGetOut,
    KvSetIn,
    KvSetOut,
)

from ..models.kv_set_in import _KvSetIn
from ..models.kv_get_in import _KvGetIn
from ..models.kv_delete_in import _KvDeleteIn


class KvAsync(ApiBase):
    async def set(
        self,
        key: str,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        body = _KvSetIn(
            key=key,
            value=kv_set_in.value,
            ttl=kv_set_in.ttl,
            behavior=kv_set_in.behavior,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/set",
            body=body,
            response_type=KvSetOut,
        )

    async def get(
        self,
        key: str,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        body = _KvGetIn(
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get",
            body=body,
            response_type=KvGetOut,
        )

    async def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get-namespace",
            body=body,
            response_type=KvGetNamespaceOut,
        )

    async def delete(
        self,
        key: str,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        body = _KvDeleteIn(
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/delete",
            body=body,
            response_type=KvDeleteOut,
        )


class Kv(ApiBase):
    def set(
        self,
        key: str,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        body = _KvSetIn(
            key=key,
            value=kv_set_in.value,
            ttl=kv_set_in.ttl,
            behavior=kv_set_in.behavior,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/set",
            body=body,
            response_type=KvSetOut,
        )

    def get(
        self,
        key: str,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        body = _KvGetIn(
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/get",
            body=body,
            response_type=KvGetOut,
        )

    def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/get-namespace",
            body=body,
            response_type=KvGetNamespaceOut,
        )

    def delete(
        self,
        key: str,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        body = _KvDeleteIn(
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/delete",
            body=body,
            response_type=KvDeleteOut,
        )
