# this file is @generated

from .common import BaseModel
from .mcp_events_delivery_mode import McpEventsDeliveryMode


class McpEventsUnsubscribeDeliveryIn(BaseModel):
    mode: McpEventsDeliveryMode
    """How events are delivered. Only `webhook` is supported."""

    url: str
    """The callback URL the subscription was made with."""
