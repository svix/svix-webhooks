# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenCreateNamespaceIn(BaseModel):
    name: str

    max_storage_bytes: t.Optional[int] = Field(default=None, alias="max_storage_bytes")
