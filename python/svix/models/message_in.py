# this file is @generated
import typing as t

from .application_in import ApplicationIn
from .common import BaseModel


class MessageIn(BaseModel):
    application: t.Optional[ApplicationIn] = None
    """Optionally creates a new application alongside the message.

    If the application id or uid that is used in the path already exists, this argument is ignored."""

    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    event_id: t.Optional[str] = None
    """Optional unique identifier for the message"""

    event_type: str
    """The event type's name"""

    payload: t.Dict[str, t.Any]
    """JSON payload to send as the request body of the webhook.

    We also support sending non-JSON payloads. Please contact us for more information."""

    payload_retention_hours: t.Optional[int] = None
    """Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`."""

    payload_retention_period: t.Optional[int] = None
    """Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`."""

    tags: t.Optional[t.List[str]] = None
    """List of free-form tags that can be filtered by when listing messages"""

    transformations_params: t.Optional[t.Dict[str, t.Any]] = None
    """Extra parameters to pass to Transformations (for future use)"""
