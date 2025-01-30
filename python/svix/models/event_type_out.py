# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class EventTypeOut(BaseModel):
    archived: t.Optional[bool] = None

    created_at: datetime

    deprecated: bool

    description: str

    feature_flag: t.Optional[str] = None

    group_name: t.Optional[str] = None
    """The event type group's name"""

    name: str
    """The event type's name"""

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""

    updated_at: datetime
