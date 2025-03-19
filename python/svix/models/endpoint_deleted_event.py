# this file is @generated

from .common import BaseModel
from .endpoint_deleted_event_data import EndpointDeletedEventData


class EndpointDeletedEvent(BaseModel):
    """Sent when an endpoint is deleted."""

    data: EndpointDeletedEventData

    type: str
