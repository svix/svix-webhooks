# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class AdminAuthTokenOut(BaseModel):
    id: str

    name: str

    created: datetime

    updated: datetime

    expiry: t.Optional[datetime] = None

    role: str

    enabled: bool
    """Whether this token is currently enabled."""
