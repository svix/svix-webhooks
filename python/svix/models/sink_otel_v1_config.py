# this file is @generated
import typing as t

from .common import BaseModel


class SinkOtelV1Config(BaseModel):
    headers: t.Optional[t.Dict[str, str]] = None

    url: str
