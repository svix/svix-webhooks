# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointSecretRotateIn(SvixBaseModel):
    key: t.Optional[str] = None
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""
