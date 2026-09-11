# this file is @generated
from datetime import datetime

from .common import BaseModel


class AutoConfigOut(BaseModel):
    created_at: datetime

    token: str

    id: str
    """The AutoConfigSubscription's ID."""
