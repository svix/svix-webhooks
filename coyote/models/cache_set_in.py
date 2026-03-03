# this file is @generated

from ..internal.base_model import BaseModel


class CacheSetIn(BaseModel):
    value: bytes

    ttl: int
    """Time to live in milliseconds"""


class _CacheSetIn(BaseModel):
    key: str

    value: bytes

    ttl: int
    """Time to live in milliseconds"""
