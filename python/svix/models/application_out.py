# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel


class ApplicationOut(SvixBaseModel):
    created_at: datetime = Field(alias="createdAt")

    id: str
    """The app's ID"""

    metadata: t.Dict[str, str]

    name: str

    rate_limit: t.Optional[int] = Field(default=None, alias="rateLimit")

    uid: t.Optional[str] = None
    """The app's UID"""

    updated_at: datetime = Field(alias="updatedAt")
