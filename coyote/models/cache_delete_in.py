# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class CacheDeleteIn(BaseModel):
    namespace: t.Optional[str] = None


class _CacheDeleteIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str
