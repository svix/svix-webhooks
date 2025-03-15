# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .endpoint_disabled_trigger import EndpointDisabledTrigger


class EndpointDisabledEventData(BaseModel):
    """Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call."""

    app_id: str
    """The Application's ID."""

    app_uid: t.Optional[str] = None
    """The Application's UID."""

    endpoint_id: str
    """The Endpoint's ID."""

    endpoint_uid: t.Optional[str] = None
    """The Endpoint's UID."""

    fail_since: t.Optional[datetime] = None

    trigger: t.Optional[EndpointDisabledTrigger] = None
