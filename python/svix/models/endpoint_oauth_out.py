# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_oauth_config_out import EndpointOauthConfigOut


class EndpointOauthOut(BaseModel):
    enabled: bool

    config: t.Optional[EndpointOauthConfigOut] = None
