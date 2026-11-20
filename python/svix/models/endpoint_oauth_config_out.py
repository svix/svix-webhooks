# this file is @generated
import typing as t

from .common import BaseModel
from .oauth2_auth_method_in_out import Oauth2AuthMethodInOut
from .oauth2_grant_type_in_out import Oauth2GrantTypeInOut


class EndpointOauthConfigOut(BaseModel):
    grant_type: Oauth2GrantTypeInOut

    auth_method: Oauth2AuthMethodInOut

    client_id: str

    token_url: str

    scopes: t.Optional[t.List[str]] = None

    extra_params: t.Optional[t.Dict[str, str]] = None

    extra_headers: t.Optional[t.Dict[str, str]] = None
