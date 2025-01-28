# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class EventTypePatch(SvixBaseModel):
    archived: t.Optional[bool] = None

    deprecated: t.Optional[bool] = None

    description: t.Optional[str] = None

    feature_flag: t.Optional[str] = Field(default=None, alias="featureFlag")

    group_name: t.Optional[str] = Field(default=None, alias="groupName")
    """The event type group's name"""

    schemas: t.Optional[t.Dict[str, t.Any]] = None
