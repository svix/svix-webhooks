# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenCreateIn(BaseModel):
    namespace: t.Optional[str] = None

    name: str

    prefix: t.Optional[str] = None

    suffix: t.Optional[str] = None

    expiry_ms: t.Optional[int] = Field(default=None, alias="expiry_ms")
    """Milliseconds from now until the token expires."""

    metadata: t.Optional[t.Dict[str, str]] = None

    owner_id: str = Field(alias="owner_id")

    scopes: t.Optional[t.List[str]] = None

    enabled: t.Optional[bool] = None
    """Whether the token is enabled. Defaults to `true`."""
