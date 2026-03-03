# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy
from .storage_type import StorageType


class CacheGetNamespaceOut(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    storage_type: StorageType = Field(alias="storage_type")

    eviction_policy: EvictionPolicy = Field(alias="eviction_policy")

    created_at: datetime = Field(alias="created_at")

    updated_at: datetime = Field(alias="updated_at")
