# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    CacheDeleteIn,
    CacheDeleteOut,
    CacheGetIn,
    CacheGetOut,
    CacheSetIn,
    CacheSetOut,
)
from .cache_namespace import (
    CacheNamespace,
    CacheNamespaceAsync,
)

from ..models.cache_set_in import _CacheSetIn
from ..models.cache_get_in import _CacheGetIn
from ..models.cache_delete_in import _CacheDeleteIn


class CacheAsync(ApiBase):
    @property
    def namespace(self) -> CacheNamespaceAsync:
        return CacheNamespaceAsync(self._client)

    async def set(
        self,
        key: str,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        body = _CacheSetIn(
            namespace=cache_set_in.namespace,
            key=key,
            value=cache_set_in.value,
            ttl=cache_set_in.ttl,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cache.set",
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
            namespace=cache_get_in.namespace,
            key=key,
            consistency=cache_get_in.consistency,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cache.get",
            body=body,
            response_type=CacheGetOut,
        )

    async def delete(
        self,
        key: str,
        cache_delete_in: CacheDeleteIn = CacheDeleteIn(),
    ) -> CacheDeleteOut:
        """Cache Delete"""
        body = _CacheDeleteIn(
            namespace=cache_delete_in.namespace,
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cache.delete",
            body=body,
            response_type=CacheDeleteOut,
        )


class Cache(ApiBase):
    @property
    def namespace(self) -> CacheNamespace:
        return CacheNamespace(self._client)

    def set(
        self,
        key: str,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        body = _CacheSetIn(
            namespace=cache_set_in.namespace,
            key=key,
            value=cache_set_in.value,
            ttl=cache_set_in.ttl,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cache.set",
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
            namespace=cache_get_in.namespace,
            key=key,
            consistency=cache_get_in.consistency,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cache.get",
            body=body,
            response_type=CacheGetOut,
        )

    def delete(
        self,
        key: str,
        cache_delete_in: CacheDeleteIn = CacheDeleteIn(),
    ) -> CacheDeleteOut:
        """Cache Delete"""
        body = _CacheDeleteIn(
            namespace=cache_delete_in.namespace,
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cache.delete",
            body=body,
            response_type=CacheDeleteOut,
        )
