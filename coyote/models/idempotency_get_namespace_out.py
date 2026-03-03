# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class IdempotencyGetNamespaceOut(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    created_at: datetime = Field(alias="created_at")

    updated_at: datetime = Field(alias="updated_at")
