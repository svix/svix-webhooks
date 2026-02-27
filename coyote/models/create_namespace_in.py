# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .retention import Retention
from .storage_type import StorageType


class CreateNamespaceIn(BaseModel):
    name: str

    retention: t.Optional[Retention] = None

    storage_type: t.Optional[StorageType] = Field(default=None, alias="storage_type")
