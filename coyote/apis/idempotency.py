# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyAbortIn,
    IdempotencyAbortOut,
    IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceOut,
)

from ..models.idempotency_abort_in import _IdempotencyAbortIn


class IdempotencyAsync(ApiBase):
    async def abort(
        self,
        key: str,
        idempotency_abort_in: IdempotencyAbortIn = IdempotencyAbortIn(),
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = _IdempotencyAbortIn(
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )

    async def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )


class Idempotency(ApiBase):
    def abort(
        self,
        key: str,
        idempotency_abort_in: IdempotencyAbortIn = IdempotencyAbortIn(),
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = _IdempotencyAbortIn(
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/idempotency/abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )

    def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )
