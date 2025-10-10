# this file is @generated
import typing as t

from .common import BaseModel


class RotatePollerTokenIn(BaseModel):
    expiry: t.Optional[int] = None
    """How long the token will be valid for, in seconds. Can be up to 31,536,000 seconds (1 year)."""

    old_token_expiry: t.Optional[int] = None
    """Updates the previous token's expiration, in seconds.

    If set to 0, the old token will immediately be revoked. Must be between 0 and 86,400 seconds (1 day).

    Defaults to 300 seconds (5 minutes)."""
