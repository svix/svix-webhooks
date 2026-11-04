# this file is @generated
import typing as t

from .common import BaseModel


class EventExampleIn(BaseModel):
    event_type: str
    """The event type's name"""

    example_index: t.Optional[int] = None
    """If the event type schema contains an array of examples, chooses which one to send.

    Defaults to the first example. Ignored if the schema doesn't contain an array of examples."""

    payload: t.Optional[t.Dict[str, t.Any]] = None
    """Custom payload to send as an example

    This is only available to allow-listed customers and should otherwise not be passed. Please
    contact us if you need access to this functionality"""
