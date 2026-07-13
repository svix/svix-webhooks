# this file is @generated
import typing as t

from .common import BaseModel


class EventTypeUpsertIn(BaseModel):
    description: str

    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""

    feature_flags: t.Optional[t.List[str]] = None

    group_name: t.Optional[str] = None
    """The event type group's name"""
