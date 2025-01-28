# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointHeadersIn(SvixBaseModel):
    headers: t.Dict[str, str]
