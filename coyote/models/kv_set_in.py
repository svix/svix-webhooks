# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .operation_behavior import OperationBehavior


class KvSetIn(BaseModel):
    namespace: t.Optional[str] = None

    value: bytes

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    behavior: t.Optional[OperationBehavior] = None

    version: t.Optional[int] = None
    """If set, the write only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""


class _KvSetIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    value: bytes

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    behavior: t.Optional[OperationBehavior] = None

    version: t.Optional[int] = None
    """If set, the write only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""
