# this file is @generated
import typing as t

from .common import BaseModel


class ApplicationIn(BaseModel):
    name: str
    """Application name for human consumption."""

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this application.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the application."""

    metadata: t.Optional[t.Dict[str, str]] = None
