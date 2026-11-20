# this file is @generated
import typing as t

from .common import BaseModel
from .oauth_jws_signing_algorithm import OauthJwsSigningAlgorithm


class ClientSecretJwtParamsIn(BaseModel):
    secret_base64: str
    """The base64-encoded secret used for signing the JWT."""

    signing_algorithm: OauthJwsSigningAlgorithm
    """The algorithm used to sign the JWT."""

    secret_id: t.Optional[str] = None
    """Optional secret identifier.

    If supplied, this will be populated in the JWT header in the `kid` field."""

    token_expiry_secs: t.Optional[int] = None
    """Optional number of seconds after which the JWT should expire.

    Defaults to 300 seconds."""
