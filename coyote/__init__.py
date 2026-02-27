# this file is @generated

from .apis import (
    Cache,
    CacheAsync,
    Health,
    HealthAsync,
    Idempotency,
    IdempotencyAsync,
    Kv,
    KvAsync,
    Msgs,
    MsgsAsync,
    RateLimiter,
    RateLimiterAsync,
    Stream,
    StreamAsync,
)
from .client_base import ClientBase
from .options import CoyoteOptions


class Coyote(ClientBase):
    @property
    def cache(self) -> Cache:
        return Cache(self._client)

    @property
    def health(self) -> Health:
        return Health(self._client)

    @property
    def idempotency(self) -> Idempotency:
        return Idempotency(self._client)

    @property
    def kv(self) -> Kv:
        return Kv(self._client)

    @property
    def msgs(self) -> Msgs:
        return Msgs(self._client)

    @property
    def rate_limiter(self) -> RateLimiter:
        return RateLimiter(self._client)

    @property
    def stream(self) -> Stream:
        return Stream(self._client)


class CoyoteAsync(ClientBase):
    @property
    def cache(self) -> CacheAsync:
        return CacheAsync(self._client)

    @property
    def health(self) -> HealthAsync:
        return HealthAsync(self._client)

    @property
    def idempotency(self) -> IdempotencyAsync:
        return IdempotencyAsync(self._client)

    @property
    def kv(self) -> KvAsync:
        return KvAsync(self._client)

    @property
    def msgs(self) -> MsgsAsync:
        return MsgsAsync(self._client)

    @property
    def rate_limiter(self) -> RateLimiterAsync:
        return RateLimiterAsync(self._client)

    @property
    def stream(self) -> StreamAsync:
        return StreamAsync(self._client)


__all__ = ["Coyote", "CoyoteAsync", "CoyoteOptions"]
