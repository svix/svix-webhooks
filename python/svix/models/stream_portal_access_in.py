# this file is @generated
import typing as t

from .common import BaseModel


class StreamPortalAccessIn(BaseModel):
    expiry: t.Optional[int] = None
    """How long the token will be valid for, in seconds.

    Valid values are between 1 hour and 7 days. The default is 7 days."""

    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the created token will have access to."""

    session_id: t.Optional[str] = None
    """An optional session ID to attach to the token.

    When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID."""
