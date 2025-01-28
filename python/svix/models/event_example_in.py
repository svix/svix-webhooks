# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class EventExampleIn(SvixBaseModel):
    event_type: str = Field(alias="eventType")
    """The event type's name"""

    example_index: t.Optional[int] = Field(default=None, alias="exampleIndex")
    """If the event type schema contains an array of examples, chooses which one to send.

    Defaults to the first example. Ignored if the schema doesn't contain an array of examples."""
