# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyCreateNamespaceIn,
    IdempotencyCreateNamespaceOut,
    IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceOut,
)


class IdempotencyNamespaceAsync(ApiBase):
    async def create(
        self,
        idempotency_create_namespace_in: IdempotencyCreateNamespaceIn,
    ) -> IdempotencyCreateNamespaceOut:
        """Create idempotency namespace"""
        body = idempotency_create_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.create",
            body=body,
            response_type=IdempotencyCreateNamespaceOut,
        )

    async def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )


class IdempotencyNamespace(ApiBase):
    def create(
        self,
        idempotency_create_namespace_in: IdempotencyCreateNamespaceIn,
    ) -> IdempotencyCreateNamespaceOut:
        """Create idempotency namespace"""
        body = idempotency_create_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.create",
            body=body,
            response_type=IdempotencyCreateNamespaceOut,
        )

    def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )
