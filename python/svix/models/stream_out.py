# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class StreamOut(BaseModel):
    id: str
    """The stream's ID."""

    uid: t.Optional[str] = None
    """The stream's UID."""

    name: t.Optional[str] = None
    """The stream's name."""

    created_at: datetime

    updated_at: datetime

    metadata: t.Dict[str, str]
