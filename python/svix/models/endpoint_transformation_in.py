# this file is @generated
import typing as t

from .common import BaseModel


class EndpointTransformationIn(BaseModel):
    code: t.Optional[str] = None

    enabled: t.Optional[bool] = None
