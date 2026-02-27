# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .rate_limit_status import RateLimitStatus


class RateLimiterCheckOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after: t.Optional[int] = Field(default=None, alias="retry_after")
    """Seconds until enough tokens are available (only present when allowed is false)"""

    status: RateLimitStatus
    """Whether the request is allowed"""
