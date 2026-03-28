# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .retention import Retention


class MsgNamespaceCreateIn(BaseModel):
    retention: t.Optional[Retention] = None


class _MsgNamespaceCreateIn(BaseModel):
    name: str

    retention: t.Optional[Retention] = None
