# this file is @generated
import typing as t

from .client_secret_jwt_params_in import ClientSecretJwtParamsIn
from .common import BaseModel
from .oauth2_auth_method_in_out import Oauth2AuthMethodInOut
from .oauth2_grant_type_in_out import Oauth2GrantTypeInOut


class EndpointOauthConfigIn(BaseModel):
    grant_type: Oauth2GrantTypeInOut
    """The OAuth grant type."""

    auth_method: Oauth2AuthMethodInOut

    jwt_params: t.Optional[ClientSecretJwtParamsIn] = None
    """Optional JWT parameters. Only required for `clientSecretJwt`"""

    client_id: str
    """The client ID. Required for all authentication types."""

    client_secret: t.Optional[str] = None
    """Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.

    For `clientSecretBasic`, the secret will be appended to the `Authorization` header.
    For `clientSecretPost`, this will be added to the body in a `client_secret` parameter."""

    token_url: str
    """The URL of the authorization server."""

    scopes: t.Optional[t.List[str]] = None
    """Optional OAuth scopes added to the request body."""

    extra_params: t.Optional[t.Dict[str, str]] = None
    """Extra parameters added to the request body as key-value pairs."""

    refresh_token: t.Optional[str] = None
    """For `refreshToken` grant type."""

    extra_headers: t.Optional[t.Dict[str, str]] = None
    """Optional headers added to the token request."""
