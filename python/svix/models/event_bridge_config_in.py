# this file is @generated
import typing as t

from .common import BaseModel


class EventBridgeConfigIn(BaseModel):
    event_bus_name: str
    """The name or ARN of the event bus to receive the event"""

    detail_type: t.Optional[str] = None
    """Free-form string, with a maximum of 128 characters"""

    access_key_id: t.Optional[str] = None
    """Access key ID.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    region: t.Optional[str] = None
    """The region of the EventBridge bus.

    Currently a required field, but marked as optional because we may infer it from other fields in the future."""
