# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class StreamEventTypeOut(BaseModel):
    archived: bool

    created_at: datetime

    deprecated: bool

    description: t.Optional[str] = None

    feature_flags: t.Optional[t.List[str]] = None

    name: str
    """The event type's name"""

    updated_at: datetime
