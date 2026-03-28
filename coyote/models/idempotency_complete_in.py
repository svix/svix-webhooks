# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class IdempotencyCompleteIn(BaseModel):
    namespace: t.Optional[str] = None

    response: bytes
    """The response to cache"""

    ttl: int
    """TTL in seconds for the cached response"""


class _IdempotencyCompleteIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    response: bytes
    """The response to cache"""

    ttl: int
    """TTL in seconds for the cached response"""
