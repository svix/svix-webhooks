# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenExpireIn(BaseModel):
    namespace: t.Optional[str] = None

    id: str

    expiry_ms: t.Optional[int] = Field(default=None, alias="expiry_ms")
    """Milliseconds from now until the token expires. `None` means expire immediately."""
