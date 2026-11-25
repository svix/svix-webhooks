# this file is @generated

from .common import BaseModel


class EndpointMtlsConfigIn(BaseModel):
    identity: str
    """A PEM encoded private key and X.509 certificate to identify the webhook sender."""
