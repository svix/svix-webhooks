# this file is @generated
import typing as t

from .common import SvixBaseModel


class EventTypePatch(SvixBaseModel):
    archived: t.Optional[bool] = None
    deprecated: t.Optional[bool] = None
    description: t.Optional[str] = None
    feature_flag: t.Optional[str] = None
    group_name: t.Optional[str] = None
    """The event type group's name"""
    schemas: t.Optional[t.Dict[str, t.Any]] = None
