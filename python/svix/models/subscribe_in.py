# this file is @generated

from .common import BaseModel
from .endpoint_in import EndpointIn


class SubscribeIn(BaseModel):
    endpoint: EndpointIn
