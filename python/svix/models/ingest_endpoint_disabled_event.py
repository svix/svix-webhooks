# this file is @generated

from .common import BaseModel
from .ingest_endpoint_disabled_event_data import IngestEndpointDisabledEventData


class IngestEndpointDisabledEvent(BaseModel):
    """Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call."""

    data: IngestEndpointDisabledEventData

    type: str
