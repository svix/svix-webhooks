# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_mtls_config_out import EndpointMtlsConfigOut


class EndpointMtlsOut(BaseModel):
    enabled: bool

    config: t.Optional[EndpointMtlsConfigOut] = None
