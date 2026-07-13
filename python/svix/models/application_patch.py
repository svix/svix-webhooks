# this file is @generated
import typing as t

from .common import BaseModel


class ApplicationPatch(BaseModel):
    name: t.Optional[str] = None

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this application.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """The Application's UID."""

    metadata: t.Optional[t.Dict[str, str]] = None
