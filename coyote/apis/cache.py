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

from ..models.cache_set_in import _CacheSetIn
from ..models.cache_get_in import _CacheGetIn
from ..models.cache_delete_in import _CacheDeleteIn


class CacheAsync(ApiBase):
    async def set(
        self,
        key: str,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        body = _CacheSetIn(
            key=key,
            value=cache_set_in.value,
            ttl=cache_set_in.ttl,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/set",
            body=body,
            response_type=CacheSetOut,
        )

    async def get(
        self,
        key: str,
        cache_get_in: CacheGetIn = CacheGetIn(),
    ) -> CacheGetOut:
        """Cache Get"""
        body = _CacheGetIn(
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get",
            body=body,
            response_type=CacheGetOut,
        )

    async def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get-namespace",
            body=body,
            response_type=CacheGetNamespaceOut,
        )

    async def delete(
        self,
        key: str,
        cache_delete_in: CacheDeleteIn = CacheDeleteIn(),
    ) -> CacheDeleteOut:
        """Cache Delete"""
        body = _CacheDeleteIn(
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/cache/delete",
            body=body,
            response_type=CacheDeleteOut,
        )


class Cache(ApiBase):
    def set(
        self,
        key: str,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        body = _CacheSetIn(
            key=key,
            value=cache_set_in.value,
            ttl=cache_set_in.ttl,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/cache/set",
            body=body,
            response_type=CacheSetOut,
        )

    def get(
        self,
        key: str,
        cache_get_in: CacheGetIn = CacheGetIn(),
    ) -> CacheGetOut:
        """Cache Get"""
        body = _CacheGetIn(
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/cache/get",
            body=body,
            response_type=CacheGetOut,
        )

    def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/cache/get-namespace",
            body=body,
            response_type=CacheGetNamespaceOut,
        )

    def delete(
        self,
        key: str,
        cache_delete_in: CacheDeleteIn = CacheDeleteIn(),
    ) -> CacheDeleteOut:
        """Cache Delete"""
        body = _CacheDeleteIn(
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/cache/delete",
            body=body,
            response_type=CacheDeleteOut,
        )
