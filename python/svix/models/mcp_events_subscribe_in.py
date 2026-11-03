# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_challenge_mode import EndpointChallengeMode
from .mcp_events_subscribe_delivery_in import McpEventsSubscribeDeliveryIn


class McpEventsSubscribeIn(BaseModel):
    name: str
    """The event to subscribe to. It must be an existing event type."""

    arguments: t.Optional[t.Dict[str, t.Any]] = None
    """The filter arguments of the `events/subscribe` request, as described by
    the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
    Together with `name` and the callback URL they identify the subscription

    They're stored in the endpoint's `mcpEventArguments` metadata, but
    aren't used to filter messages; use `channels` for that.

    See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
    in OpenAI's MCP Events docs."""

    delivery: McpEventsSubscribeDeliveryIn

    ttl_ms: t.Optional[int] = None
    """The requested lifetime of the subscription, in milliseconds. Omit for
    the default of 7 days, or set to `null` for a subscription that never
    expires."""

    secret_grace_period_seconds: t.Optional[int] = None
    """How long the previous secret keeps signing when a refresh changes it,
    in seconds.

    Valid values are between 0 (immediate expiry) and 7 days. The default
    is 5 minutes."""

    challenge_mode: t.Optional[EndpointChallengeMode] = None
    """How the callback must respond to the verification challenge."""

    channels: t.Optional[t.List[str]] = None
    """Only send messages with one of these channels to the subscription,
    for example ones derived from `arguments`. Omit to send all messages
    of the event type."""
