# this file is @generated
import typing as t

from .common import BaseModel


class OperationalWebhookEndpointIn(BaseModel):
    description: t.Optional[str] = None

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = None

    metadata: t.Optional[t.Dict[str, str]] = None

    rate_limit: t.Optional[int] = None

    secret: t.Optional[str] = None
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    url: str
