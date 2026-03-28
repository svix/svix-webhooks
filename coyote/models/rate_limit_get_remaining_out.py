# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class RateLimitGetRemainingOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: t.Optional[int] = Field(default=None, alias="retry_after_ms")
    """Milliseconds until at least one token is available (only present when remaining is 0)"""
