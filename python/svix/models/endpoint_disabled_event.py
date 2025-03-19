# this file is @generated

from .common import BaseModel
from .endpoint_disabled_event_data import EndpointDisabledEventData


class EndpointDisabledEvent(BaseModel):
    """Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call."""

    data: EndpointDisabledEventData

    type: str
