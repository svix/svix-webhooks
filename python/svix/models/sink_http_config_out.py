# this file is @generated

from .common import BaseModel
from .endpoint_headers_out import EndpointHeadersOut


class SinkHttpConfigOut(BaseModel):
    url: str

    headers: EndpointHeadersOut
