# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceIn(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    eviction_policy: t.Optional[EvictionPolicy] = Field(
        default=None, alias="eviction_policy"
    )
