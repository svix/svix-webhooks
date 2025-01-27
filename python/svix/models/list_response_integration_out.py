# this file is @generated
import typing as t

from .common import SvixBaseModel
from .integration_out import IntegrationOut


class ListResponseIntegrationOut(SvixBaseModel):
    data: t.List[IntegrationOut]
    done: bool
    iterator: str
    prev_iterator: t.Optional[str] = None
