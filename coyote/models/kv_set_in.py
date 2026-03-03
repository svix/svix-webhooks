# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .operation_behavior import OperationBehavior


class KvSetIn(BaseModel):
    value: bytes

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    behavior: t.Optional[OperationBehavior] = None


class _KvSetIn(BaseModel):
    key: str

    value: bytes

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    behavior: t.Optional[OperationBehavior] = None
