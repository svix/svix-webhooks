# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel


class EventTypeOut(SvixBaseModel):
    archived: t.Optional[bool] = None

    created_at: datetime = Field(alias="createdAt")

    deprecated: bool

    description: str

    feature_flag: t.Optional[str] = Field(default=None, alias="featureFlag")

    group_name: t.Optional[str] = Field(default=None, alias="groupName")
    """The event type group's name"""

    name: str
    """The event type's name"""

    schemas: t.Optional[t.Dict[str, t.Any]] = None
    """The schema for the event type for a specific version as a JSON schema."""

    updated_at: datetime = Field(alias="updatedAt")
