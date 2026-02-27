# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .storage_type import StorageType


class KvGetNamespaceOut(BaseModel):
    created_at: datetime = Field(alias="created_at")

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    name: str

    storage_type: StorageType = Field(alias="storage_type")

    updated_at: datetime = Field(alias="updated_at")
