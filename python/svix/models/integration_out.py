# this file is @generated
from datetime import datetime

from .common import SvixBaseModel


class IntegrationOut(SvixBaseModel):
    created_at: datetime
    id: str
    """The integ's ID"""
    name: str
    updated_at: datetime
