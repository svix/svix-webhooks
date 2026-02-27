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


class KvAsync(ApiBase):
    async def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/set",
            body=kv_set_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvSetOut,
        )

    async def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get",
            body=kv_get_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvGetOut,
        )

    async def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get-namespace",
            body=kv_get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvGetNamespaceOut,
        )

    async def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/delete",
            body=kv_delete_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvDeleteOut,
        )


class Kv(ApiBase):
    def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        return self._request_sync(
            method="post",
            path="/api/v1/kv/set",
            body=kv_set_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvSetOut,
        )

    def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        return self._request_sync(
            method="post",
            path="/api/v1/kv/get",
            body=kv_get_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvGetOut,
        )

    def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        return self._request_sync(
            method="post",
            path="/api/v1/kv/get-namespace",
            body=kv_get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvGetNamespaceOut,
        )

    def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        return self._request_sync(
            method="post",
            path="/api/v1/kv/delete",
            body=kv_delete_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=KvDeleteOut,
        )
