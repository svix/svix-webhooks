# this file is @generated
import typing as t

from .common import BaseModel


class EventTypeIn(BaseModel):
    name: str
    """The event type's name"""

    description: str

    description_translations: t.Optional[t.Dict[str, str]] = None
    """Translated descriptions keyed by BCP 47 language tag."""

    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""

    group_name: t.Optional[str] = None
    """The event type group's name"""

    feature_flags: t.Optional[t.List[str]] = None
