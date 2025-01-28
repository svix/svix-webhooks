# this file is @generated
import typing as t

from .common import SvixBaseModel


class EndpointHeadersOut(SvixBaseModel):
    """The value of the headers is returned in the `headers` field.

    Sensitive headers that have been redacted are returned in the sensitive field."""

    headers: t.Dict[str, str]

    sensitive: t.List[str]
