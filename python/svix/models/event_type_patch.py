# this file is @generated
import typing as t

from .common import BaseModel


class EventTypePatch(BaseModel):
    description: t.Optional[str] = None

    description_translations: t.Optional[t.Dict[str, str]] = None
    """Translated descriptions keyed by BCP 47 language tag."""

    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    schemas: t.Optional[t.Dict[str, t.Any]] = None

    feature_flags: t.Optional[t.List[str]] = None

    group_name: t.Optional[str] = None
    """The event type group's name"""
