# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class OperationalWebhookEndpointOut(BaseModel):
    id: str
    """The Endpoint's ID."""

    url: str

    description: str

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    disabled: t.Optional[bool] = None

    event_types: t.Optional[t.List[str]] = None

    created_at: datetime

    updated_at: datetime

    metadata: t.Dict[str, str]
