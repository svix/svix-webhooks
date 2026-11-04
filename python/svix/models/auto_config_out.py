# this file is @generated
from datetime import datetime

from .common import BaseModel


class AutoConfigOut(BaseModel):
    created_at: datetime

    token: str
    """The AutoConfig token"""

    id: str
    """The AutoConfigSubscription's ID."""
