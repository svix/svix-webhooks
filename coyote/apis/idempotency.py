# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyAbortIn,
    IdempotencyAbortOut,
    IdempotencyCompleteIn,
    IdempotencyCompleteOut,
    IdempotencyStartIn,
    IdempotencyStartOut,
)
from .idempotency_namespace import (
    IdempotencyNamespace,
    IdempotencyNamespaceAsync,
)

from ..models.idempotency_start_in import _IdempotencyStartIn
from ..models.idempotency_complete_in import _IdempotencyCompleteIn
from ..models.idempotency_abort_in import _IdempotencyAbortIn


class IdempotencyAsync(ApiBase):
    @property
    def namespace(self) -> IdempotencyNamespaceAsync:
        return IdempotencyNamespaceAsync(self._client)

    async def start(
        self,
        key: str,
        idempotency_start_in: IdempotencyStartIn,
    ) -> IdempotencyStartOut:
        """Start an idempotent request"""
        body = _IdempotencyStartIn(
            namespace=idempotency_start_in.namespace,
            key=key,
            ttl=idempotency_start_in.ttl,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.start",
            body=body,
            response_type=IdempotencyStartOut,
        )

    async def complete(
        self,
        key: str,
        idempotency_complete_in: IdempotencyCompleteIn,
    ) -> IdempotencyCompleteOut:
        """Complete an idempotent request with a response"""
        body = _IdempotencyCompleteIn(
            namespace=idempotency_complete_in.namespace,
            key=key,
            response=idempotency_complete_in.response,
            ttl=idempotency_complete_in.ttl,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.complete",
            body=body,
            response_type=IdempotencyCompleteOut,
        )

    async def abort(
        self,
        key: str,
        idempotency_abort_in: IdempotencyAbortIn = IdempotencyAbortIn(),
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = _IdempotencyAbortIn(
            namespace=idempotency_abort_in.namespace,
            key=key,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )


class Idempotency(ApiBase):
    @property
    def namespace(self) -> IdempotencyNamespace:
        return IdempotencyNamespace(self._client)

    def start(
        self,
        key: str,
        idempotency_start_in: IdempotencyStartIn,
    ) -> IdempotencyStartOut:
        """Start an idempotent request"""
        body = _IdempotencyStartIn(
            namespace=idempotency_start_in.namespace,
            key=key,
            ttl=idempotency_start_in.ttl,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.start",
            body=body,
            response_type=IdempotencyStartOut,
        )

    def complete(
        self,
        key: str,
        idempotency_complete_in: IdempotencyCompleteIn,
    ) -> IdempotencyCompleteOut:
        """Complete an idempotent request with a response"""
        body = _IdempotencyCompleteIn(
            namespace=idempotency_complete_in.namespace,
            key=key,
            response=idempotency_complete_in.response,
            ttl=idempotency_complete_in.ttl,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.complete",
            body=body,
            response_type=IdempotencyCompleteOut,
        )

    def abort(
        self,
        key: str,
        idempotency_abort_in: IdempotencyAbortIn = IdempotencyAbortIn(),
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = _IdempotencyAbortIn(
            namespace=idempotency_abort_in.namespace,
            key=key,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )
