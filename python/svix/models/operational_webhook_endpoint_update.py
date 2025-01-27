# this file is @generated
import typing as t

from .common import SvixBaseModel


class OperationalWebhookEndpointUpdate(SvixBaseModel):
    description: t.Optional[str] = None
    disabled: t.Optional[bool] = None
    filter_types: t.Optional[t.Set[str]] = None
    metadata: t.Optional[t.Dict[str, str]] = None
    rate_limit: t.Optional[int] = None
    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""
    url: str
