# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .retention import Retention
from .storage_type import StorageType


class MsgNamespaceCreateIn(BaseModel):
    retention: t.Optional[Retention] = None

    storage_type: t.Optional[StorageType] = Field(default=None, alias="storage_type")


class _MsgNamespaceCreateIn(BaseModel):
    name: str

    retention: t.Optional[Retention] = None

    storage_type: t.Optional[StorageType] = Field(default=None, alias="storage_type")
