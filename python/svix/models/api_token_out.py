# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class ApiTokenOut(BaseModel):
    token: str

    id: str

    name: t.Optional[str] = None

    created_at: datetime

    expires_at: t.Optional[datetime] = None

    scopes: t.Optional[t.List[str]] = None
