# this file is @generated

from .common import BaseModel


class MessagePrecheckOut(BaseModel):
    active: bool
    """Whether there are any active endpoint that would get sent such a message."""
