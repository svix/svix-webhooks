# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointUpdate(SvixBaseModel):
    channels: t.Optional[t.Set[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""
    description: t.Optional[str] = None
    disabled: t.Optional[bool] = None
    filter_types: t.Optional[t.Set[str]] = None
    metadata: t.Optional[t.Dict[str, str]] = None
    rate_limit: t.Optional[int] = None
    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""
    url: str
    version: t.Optional[int] = None
