# this file is @generated
import typing as t

from .common import BaseModel


class ApplicationIn(BaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: str
    """Application name for human consumption."""

    rate_limit: t.Optional[int] = None
    """Deprecated, use `throttleRate` instead."""

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this application.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the application."""
