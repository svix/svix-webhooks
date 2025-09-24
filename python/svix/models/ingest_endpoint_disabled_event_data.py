# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .endpoint_disabled_trigger import EndpointDisabledTrigger


class IngestEndpointDisabledEventData(BaseModel):
    """Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call."""

    endpoint_id: str
    """The Endpoint's ID."""

    endpoint_uid: t.Optional[str] = None
    """The Endpoint's UID."""

    fail_since: t.Optional[datetime] = None

    source_id: str
    """The Source's ID."""

    trigger: t.Optional[EndpointDisabledTrigger] = None
