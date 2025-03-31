# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class ApiTokenCensoredOut(BaseModel):
    censored_token: str

    created_at: datetime

    expires_at: t.Optional[datetime] = None

    id: str
    """The ApplicationToken's ID."""

    name: t.Optional[str] = None

    scopes: t.Optional[t.List[str]] = None
