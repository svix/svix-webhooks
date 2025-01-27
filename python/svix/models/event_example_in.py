# this file is @generated
import typing as t

from .common import SvixBaseModel


class EventExampleIn(SvixBaseModel):
    event_type: str
    """The event type's name"""
    example_index: t.Optional[int] = None
    """If the event type schema contains an array of examples, chooses which one to send.

    Defaults to the first example. Ignored if the schema doesn't contain an array of examples."""
