# this file is @generated
import typing as t

from .common import BaseModel


class StreamPatch(BaseModel):
    description: t.Optional[str] = None
    """The Stream's description."""

    metadata: t.Optional[t.Dict[str, str]] = None

    uid: t.Optional[str] = None
    """An optional unique identifier for the stream."""
