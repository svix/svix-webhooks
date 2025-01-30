# this file is @generated
from datetime import datetime

from .common import BaseModel


class IntegrationOut(BaseModel):
    created_at: datetime

    id: str
    """The integ's ID"""

    name: str

    updated_at: datetime
