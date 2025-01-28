# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class EventTypeFromOpenApi(SvixBaseModel):
    deprecated: bool

    description: str

    feature_flag: t.Optional[str] = Field(default=None, alias="featureFlag")

    group_name: t.Optional[str] = Field(default=None, alias="groupName")
    """The event type group's name"""

    name: str
    """The event type's name"""

    schemas: t.Optional[t.Dict[str, t.Any]] = None
