# this file is @generated
from enum import Enum


class Oauth2AuthMethodInOut(str, Enum):
    """The method used for authenticating to the OAuth authorization server.

    `clientSecretJwt` will construct a JWT used for authentication
    with the oauth authorization server. This method is less
    commonly used and may not be supported by all oauth providers.
    `clientSecretBasic` will authenticate to the oauth authorization server
    using an `Authorization` header with the client secret as the value.
    This is the most common means of authentication.
    `clientSecretPost` will authenticate to the oauth authorization server
    by passing the client secret in a `client_secret` field in the
    request body. This method may not be supported by all oauth providers,
    and in general `clientSecretBasic` should be preferred."""

    CLIENT_SECRET_JWT = "clientSecretJwt"
    CLIENT_SECRET_BASIC = "clientSecretBasic"
    CLIENT_SECRET_POST = "clientSecretPost"

    def __str__(self) -> str:
        return str(self.value)
