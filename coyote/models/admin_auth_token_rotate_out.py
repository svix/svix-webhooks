# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel


class AdminAuthTokenRotateOut(BaseModel):
    id: str

    token: str

    created: datetime

    updated: datetime
