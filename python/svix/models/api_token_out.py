# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class ApiTokenOut(BaseModel):
    created_at: datetime

    expires_at: t.Optional[datetime] = None

    id: str
    """The GlobalApplicationToken's ID."""

    name: t.Optional[str] = None

    scopes: t.Optional[t.List[str]] = None

    token: str
