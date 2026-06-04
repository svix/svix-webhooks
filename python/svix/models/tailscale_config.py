# this file is @generated
import typing as t

from .common import BaseModel


class TailscaleConfig(BaseModel):
    secret: str
    """Shared secret for Tailscale Webhooks"""

    timestamp_grace_seconds: t.Optional[int] = None
    """Grace period (in seconds) for the timestamp.

    If not passed, timestamp age will not be checked."""
