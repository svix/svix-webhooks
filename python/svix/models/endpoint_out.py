# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel


class EndpointOut(SvixBaseModel):
    channels: t.Optional[t.List[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""

    created_at: datetime = Field(alias="createdAt")

    description: str
    """An example endpoint name."""

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = Field(default=None, alias="filterTypes")

    id: str
    """The ep's ID"""

    metadata: t.Dict[str, str]

    rate_limit: t.Optional[int] = Field(default=None, alias="rateLimit")

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    updated_at: datetime = Field(alias="updatedAt")

    url: str

    version: int
