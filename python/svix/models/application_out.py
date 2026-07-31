# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class ApplicationOut(BaseModel):
    uid: t.Optional[str] = None
    """Optional unique identifier for the application."""

    name: str
    """Application name for human consumption."""

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this application.

    Outgoing messages will be throttled to this rate."""

    id: str
    """The Application's ID."""

    created_at: datetime

    updated_at: datetime

    metadata: t.Dict[str, str]
