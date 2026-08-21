# this file is @generated
import typing as t

from .common import BaseModel


class OtelTracingConfigIn(BaseModel):
    url: str

    headers: t.Optional[t.Dict[str, str]] = None
