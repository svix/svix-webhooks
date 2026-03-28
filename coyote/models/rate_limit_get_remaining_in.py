# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .rate_limit_token_bucket_config import RateLimitTokenBucketConfig


class RateLimitGetRemainingIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    config: RateLimitTokenBucketConfig
    """Rate limiter configuration"""
