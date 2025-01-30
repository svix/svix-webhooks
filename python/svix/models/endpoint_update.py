# this file is @generated
import typing as t

from .common import BaseModel


class EndpointUpdate(BaseModel):
    channels: t.Optional[t.List[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""

    description: t.Optional[str] = None

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = None

    metadata: t.Optional[t.Dict[str, str]] = None

    rate_limit: t.Optional[int] = None

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    url: str

    version: t.Optional[int] = None
