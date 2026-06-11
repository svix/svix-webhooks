# this file is @generated
import typing as t

from .auto_config_sink_type import AutoConfigSinkType
from .common import BaseModel
from .endpoint_in import EndpointIn


class SubscribeIn(BaseModel):
    endpoint: t.Optional[EndpointIn] = None

    sink: t.Optional[AutoConfigSinkType] = None
