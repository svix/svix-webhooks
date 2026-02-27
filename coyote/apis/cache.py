# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    CacheDeleteIn,
    CacheDeleteOut,
    CacheGetIn,
    CacheGetNamespaceIn,
    CacheGetNamespaceOut,
    CacheGetOut,
    CacheSetIn,
    CacheSetOut,
)


class CacheAsync(ApiBase):
    async def set(
        self,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/set",
            body=cache_set_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheSetOut,
        )

    async def get(
        self,
        cache_get_in: CacheGetIn,
    ) -> CacheGetOut:
        """Cache Get"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get",
            body=cache_get_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheGetOut,
        )

    async def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get-namespace",
            body=cache_get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheGetNamespaceOut,
        )

    async def delete(
        self,
        cache_delete_in: CacheDeleteIn,
    ) -> CacheDeleteOut:
        """Cache Delete"""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/delete",
            body=cache_delete_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheDeleteOut,
        )


class Cache(ApiBase):
    def set(
        self,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        return self._request_sync(
            method="post",
            path="/api/v1/cache/set",
            body=cache_set_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheSetOut,
        )

    def get(
        self,
        cache_get_in: CacheGetIn,
    ) -> CacheGetOut:
        """Cache Get"""
        return self._request_sync(
            method="post",
            path="/api/v1/cache/get",
            body=cache_get_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheGetOut,
        )

    def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        return self._request_sync(
            method="post",
            path="/api/v1/cache/get-namespace",
            body=cache_get_namespace_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheGetNamespaceOut,
        )

    def delete(
        self,
        cache_delete_in: CacheDeleteIn,
    ) -> CacheDeleteOut:
        """Cache Delete"""
        return self._request_sync(
            method="post",
            path="/api/v1/cache/delete",
            body=cache_delete_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=CacheDeleteOut,
        )
