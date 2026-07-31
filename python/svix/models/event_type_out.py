# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class EventTypeOut(BaseModel):
    name: str
    """The event type's name"""

    description: str

    archived: t.Optional[bool] = None

    deprecated: bool

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""

    created_at: datetime

    updated_at: datetime

    group_name: t.Optional[str] = None
    """The event type group's name"""

    feature_flags: t.Optional[t.List[str]] = None

    feature_flag: t.Optional[str] = None
