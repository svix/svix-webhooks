# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    RateLimitCreateNamespaceIn,
    RateLimitCreateNamespaceOut,
    RateLimitGetNamespaceIn,
    RateLimitGetNamespaceOut,
)


class RateLimitNamespaceAsync(ApiBase):
    async def create(
        self,
        rate_limit_create_namespace_in: RateLimitCreateNamespaceIn,
    ) -> RateLimitCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limit_create_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.rate-limit.namespace.create",
            body=body,
            response_type=RateLimitCreateNamespaceOut,
        )

    async def get(
        self,
        rate_limit_get_namespace_in: RateLimitGetNamespaceIn,
    ) -> RateLimitGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limit_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.rate-limit.namespace.get",
            body=body,
            response_type=RateLimitGetNamespaceOut,
        )


class RateLimitNamespace(ApiBase):
    def create(
        self,
        rate_limit_create_namespace_in: RateLimitCreateNamespaceIn,
    ) -> RateLimitCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limit_create_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.rate-limit.namespace.create",
            body=body,
            response_type=RateLimitCreateNamespaceOut,
        )

    def get(
        self,
        rate_limit_get_namespace_in: RateLimitGetNamespaceIn,
    ) -> RateLimitGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limit_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.rate-limit.namespace.get",
            body=body,
            response_type=RateLimitGetNamespaceOut,
        )
