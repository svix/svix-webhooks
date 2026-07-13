# this file is @generated
import typing as t

from .common import BaseModel


class EndpointSecretRotateIn(BaseModel):
    key: t.Optional[str] = None
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""

    grace_period_seconds: t.Optional[int] = None
    """How long the old secret will be valid for, in seconds.

    Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours."""
