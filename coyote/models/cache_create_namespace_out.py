# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceOut(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    eviction_policy: EvictionPolicy = Field(alias="eviction_policy")

    created: datetime

    updated: datetime
