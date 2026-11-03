# this file is @generated

from .common import BaseModel
from .mcp_events_delivery_mode import McpEventsDeliveryMode


class McpEventsSubscribeDeliveryIn(BaseModel):
    mode: McpEventsDeliveryMode
    """How events are delivered. Only `webhook` is supported."""

    url: str
    """The callback URL events are sent to."""

    secret: str
    """The secret events are signed with. MCP Events requires a `whsec_`
    secret."""
