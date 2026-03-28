# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AdminAuthTokenUpdateIn(BaseModel):
    id: str

    name: t.Optional[str] = None

    expiry_ms: t.Optional[int] = Field(default=None, alias="expiry_ms")

    enabled: t.Optional[bool] = None
