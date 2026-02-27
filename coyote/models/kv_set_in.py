# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .operation_behavior import OperationBehavior


class KvSetIn(BaseModel):
    behavior: t.Optional[OperationBehavior] = None

    key: str

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    value: t.List[int]
