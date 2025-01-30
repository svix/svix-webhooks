# this file is @generated
import typing as t

from .common import BaseModel


class EndpointHeadersPatchIn(BaseModel):
    headers: t.Dict[str, str]
