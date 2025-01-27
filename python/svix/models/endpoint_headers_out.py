# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointHeadersOut(SvixBaseModel):
    headers: t.Dict[str, str]
    sensitive: t.Set[str]
