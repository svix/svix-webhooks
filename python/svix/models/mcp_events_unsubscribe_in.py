# this file is @generated
import typing as t

from .common import BaseModel
from .mcp_events_unsubscribe_delivery_in import McpEventsUnsubscribeDeliveryIn


class McpEventsUnsubscribeIn(BaseModel):
    name: str
    """The event the subscription was made for."""

    arguments: t.Optional[t.Dict[str, t.Any]] = None
    """The filter arguments the subscription was made with, as given to
    `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
    in OpenAI's MCP Events docs."""

    delivery: McpEventsUnsubscribeDeliveryIn
