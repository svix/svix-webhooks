# this file is @generated

from .apis import (
    Admin,
    AdminAsync,
    AuthToken,
    AuthTokenAsync,
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
    RateLimit,
    RateLimitAsync,
)
from .client_base import ClientBase
from .options import CoyoteOptions


class Coyote(ClientBase):
    @property
    def admin(self) -> Admin:
        return Admin(self._client)

    @property
    def auth_token(self) -> AuthToken:
        return AuthToken(self._client)

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
    def rate_limit(self) -> RateLimit:
        return RateLimit(self._client)


class CoyoteAsync(ClientBase):
    @property
    def admin(self) -> AdminAsync:
        return AdminAsync(self._client)

    @property
    def auth_token(self) -> AuthTokenAsync:
        return AuthTokenAsync(self._client)

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
    def rate_limit(self) -> RateLimitAsync:
        return RateLimitAsync(self._client)


__all__ = ["Coyote", "CoyoteAsync", "CoyoteOptions"]
