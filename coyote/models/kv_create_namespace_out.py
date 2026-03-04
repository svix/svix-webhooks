# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .storage_type import StorageType


class KvCreateNamespaceOut(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    storage_type: StorageType = Field(alias="storage_type")

    created: datetime

    updated: datetime
