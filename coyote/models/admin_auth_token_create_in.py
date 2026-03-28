# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AdminAuthTokenCreateIn(BaseModel):
    name: str

    role: str

    expiry_ms: t.Optional[int] = Field(default=None, alias="expiry_ms")
    """Milliseconds from now until the token expires."""

    enabled: t.Optional[bool] = None
    """Whether the token is enabled. Defaults to `true`."""
