# this file is @generated
import typing as t

from .common import BaseModel


class MessagePrecheckIn(BaseModel):
    event_type: str
    """The event type's name"""

    channels: t.Optional[t.List[str]] = None
