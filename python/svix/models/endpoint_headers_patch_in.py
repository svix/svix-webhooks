# this file is @generated
import typing as t

from .common import BaseModel


class EndpointHeadersPatchIn(BaseModel):
    delete_headers: t.Optional[t.List[str]] = None
    """A list of headers be be removed"""

    headers: t.Dict[str, str]
