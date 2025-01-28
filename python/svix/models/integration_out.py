# this file is @generated
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel


class IntegrationOut(SvixBaseModel):
    created_at: datetime = Field(alias="createdAt")

    id: str
    """The integ's ID"""

    name: str

    updated_at: datetime = Field(alias="updatedAt")
