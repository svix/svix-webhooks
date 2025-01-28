# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointTransformationIn(SvixBaseModel):
    code: t.Optional[str] = None

    enabled: t.Optional[bool] = None
