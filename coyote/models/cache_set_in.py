# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class CacheSetIn(BaseModel):
    key: str

    ttl: int
    """Time to live in milliseconds"""

    value: t.List[int]
