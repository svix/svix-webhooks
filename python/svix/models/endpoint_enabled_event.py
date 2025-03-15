# this file is @generated

from .common import BaseModel
from .endpoint_enabled_event_data import EndpointEnabledEventData


class EndpointEnabledEvent(BaseModel):
    """Sent when an endpoint has been enabled."""

    data: EndpointEnabledEventData

    type: str
