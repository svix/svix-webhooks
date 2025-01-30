# this file is @generated

from .common import BaseModel


class EndpointSecretOut(BaseModel):
    key: str
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""
