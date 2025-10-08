# this file is @generated
import typing as t

from .common import BaseModel


class StreamEventTypePatch(BaseModel):
    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    description: t.Optional[str] = None

    feature_flags: t.Optional[t.List[str]] = None

    name: t.Optional[str] = None
    """The event type's name"""
