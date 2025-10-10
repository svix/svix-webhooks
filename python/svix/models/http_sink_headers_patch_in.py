# this file is @generated
import typing as t

from .common import BaseModel


class HttpSinkHeadersPatchIn(BaseModel):
    headers: t.Dict[str, str]
