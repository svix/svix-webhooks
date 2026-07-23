# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class StreamEventTypeOut(BaseModel):
    name: str
    """The event type's name"""

    description: t.Optional[str] = None

    created_at: datetime

    updated_at: datetime

    deprecated: bool

    archived: bool

    feature_flags: t.Optional[t.List[str]] = None
