# this file is @generated

from .common import BaseModel
from .endpoint_updated_event_data import EndpointUpdatedEventData


class EndpointUpdatedEvent(BaseModel):
    """Sent when an endpoint is updated."""

    data: EndpointUpdatedEventData

    type: str
