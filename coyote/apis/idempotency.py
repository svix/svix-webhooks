# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyAbortIn,
    IdempotencyAbortOut,
)
from .idempotency_namespace import (
    IdempotencyNamespace,
    IdempotencyNamespaceAsync,
)

from ..models.idempotency_abort_in import _IdempotencyAbortIn


class IdempotencyAsync(ApiBase):
    @property
    def namespace(self) -> IdempotencyNamespaceAsync:
        return IdempotencyNamespaceAsync(self._client)

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


class Idempotency(ApiBase):
    @property
    def namespace(self) -> IdempotencyNamespace:
        return IdempotencyNamespace(self._client)

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
