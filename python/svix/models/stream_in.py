# this file is @generated
import typing as t

from .common import BaseModel


class StreamIn(BaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: str
    """The stream's name."""

    uid: t.Optional[str] = None
    """An optional unique identifier for the stream."""
