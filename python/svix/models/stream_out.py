# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class StreamOut(BaseModel):
    created_at: datetime

    id: str
    """The stream's ID."""

    metadata: t.Dict[str, str]

    name: t.Optional[str] = None
    """The stream's name."""

    uid: t.Optional[str] = None
    """The stream's UID."""

    updated_at: datetime
