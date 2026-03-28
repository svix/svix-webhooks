# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .rate_limit_token_bucket_config import RateLimitTokenBucketConfig


class RateLimitCheckIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    tokens: t.Optional[int] = None
    """Number of tokens to consume (default: 1)"""

    config: RateLimitTokenBucketConfig
    """Rate limiter configuration"""
