# this file is @generated
import typing as t

from .common import BaseModel


class EndpointTransformationPatch(BaseModel):
    code: t.Optional[str] = None

    enabled: t.Optional[bool] = None
