# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class RateLimitCreateNamespaceOut(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")

    created: datetime

    updated: datetime
