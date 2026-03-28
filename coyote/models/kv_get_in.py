# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .consistency import Consistency


class KvGetIn(BaseModel):
    namespace: t.Optional[str] = None

    consistency: t.Optional[Consistency] = None


class _KvGetIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    consistency: t.Optional[Consistency] = None
