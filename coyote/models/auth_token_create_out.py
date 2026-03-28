# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel


class AuthTokenCreateOut(BaseModel):
    id: str

    created: datetime

    updated: datetime

    token: str
