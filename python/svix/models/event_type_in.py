# this file is @generated
import typing as t

from .common import BaseModel


class EventTypeIn(BaseModel):
    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    description: str

    feature_flag: t.Optional[str] = None

    group_name: t.Optional[str] = None
    """The event type group's name"""

    name: str
    """The event type's name"""

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""
