# this file is @generated
import typing as t

from .common import BaseModel


class EventTypeFromOpenApi(BaseModel):
    name: str
    """The event type's name"""

    description: str

    schemas: t.Optional[t.Dict[str, t.Any]] = None

    deprecated: bool

    group_name: t.Optional[str] = None
    """The event type group's name"""

    feature_flags: t.Optional[t.List[str]] = None
