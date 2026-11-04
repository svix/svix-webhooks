# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class McpEventsSubscribeOut(BaseModel):
    id: str
    """The subscription's ID, which is also the UID of its endpoint."""

    endpoint_id: str
    """The Endpoint's ID."""

    refresh_before: t.Optional[datetime] = None
    """When the subscription expires, or `null` if it never does."""

    cursor: t.Optional[str] = None
    """Always `null`, since events can't be replayed."""

    truncated: bool
    """Always `false`, since events can't be replayed."""
