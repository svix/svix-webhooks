# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class RateLimiterGetRemainingOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after: t.Optional[int] = Field(default=None, alias="retry_after")
    """Seconds until at least one token is available (only present when remaining is 0)"""
