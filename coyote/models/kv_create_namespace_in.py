# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .storage_type import StorageType


class KvCreateNamespaceIn(BaseModel):
    name: str

    storage_type: t.Optional[StorageType] = Field(default=None, alias="storage_type")

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")
