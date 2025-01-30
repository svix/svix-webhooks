# this file is @generated
import typing as t

from .common import BaseModel


class EndpointHeadersIn(BaseModel):
    headers: t.Dict[str, str]
