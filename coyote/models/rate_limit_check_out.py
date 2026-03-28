# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class RateLimitCheckOut(BaseModel):
    allowed: bool
    """Whether the request is allowed"""

    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: t.Optional[int] = Field(default=None, alias="retry_after_ms")
    """Milliseconds until enough tokens are available (only present when allowed is false)"""
