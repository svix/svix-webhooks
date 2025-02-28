# this file is @generated
import typing as t

from .common import BaseModel


class IngestEndpointHeadersIn(BaseModel):
    headers: t.Dict[str, str]
