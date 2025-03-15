# this file is @generated

from .common import BaseModel
from .endpoint_created_event_data import EndpointCreatedEventData


class EndpointCreatedEvent(BaseModel):
    """Sent when an endpoint is created."""

    data: EndpointCreatedEventData

    type: str
