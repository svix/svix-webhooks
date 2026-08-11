# this file is @generated

from .common import BaseModel
from .endpoint_headers_out import EndpointHeadersOut


class SinkOtelTracingConfigOut(BaseModel):
    url: str

    headers: EndpointHeadersOut
